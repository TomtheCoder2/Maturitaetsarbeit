use image::ImageBuffer;
use image::Rgb;
use image::{DynamicImage, RgbImage};
extern crate ocl;
use ocl::Buffer;
use ocl::ProQue;
// Camera matrix and distortion coefficients from calibration
pub const CAMERA_MATRIX: [[f64; 3]; 3] = [
    [458.11615111, 0.0, 367.97913048],
    [0.0, 458.19122834, 272.04155943],
    [0.0, 0.0, 1.0],
];
pub const DIST_COEFFS: [f64; 5] = [-0.32281387, 0.17342487, -0.0012062, 0.004915, -0.07841724];

pub struct DistortionCorrection {
    pub pro_que: ocl::ProQue,
    img_buffer: Buffer<u8>,
    output_buffer: Buffer<u8>,
}

impl DistortionCorrection {
    pub fn new() -> ocl::Result<Self> {
        let src = r#"
            __constant float CAMERA_MATRIX[9] = {
                458.11615111, 0.0, 367.97913048,
                0.0, 458.19122834, 272.04155943,
                0.0, 0.0, 1.0
            };

            __constant float DIST_COEFFS[5] = {-0.32281387, 0.17342487, -0.0012062, 0.004915, -0.07841724};

            __kernel void undistort(__global uchar* buffer, __global uchar* img) {
                int x = get_global_id(0);
                int y = get_global_id(1);

                const int width = 728;
                const int height = 544;

                if (x >= width || y >= height) return;

                float fx = CAMERA_MATRIX[0];
                float fy = CAMERA_MATRIX[4];
                float cx = CAMERA_MATRIX[2];
                float cy = CAMERA_MATRIX[5];

                // Distortion coefficients
                float k1 = DIST_COEFFS[0];
                float k2 = DIST_COEFFS[1];
                float p1 = DIST_COEFFS[2];
                float p2 = DIST_COEFFS[3];
                float k3 = DIST_COEFFS[4];

                // Normalize coordinates to [-1, 1]
                float x_normalized = (x - cx) / fx;
                float y_normalized = (y - cy) / fy;

                // Calculate radial distance
                float r2 = x_normalized * x_normalized + y_normalized * y_normalized;
                float r4 = r2 * r2;

                // Apply radial distortion
                float radial_distortion = 1.0 + k1 * r2 + k2 * r4 + k3 * r4 * r2;
                float x_radial = x_normalized * radial_distortion;
                float y_radial = y_normalized * radial_distortion;

                // Apply tangential distortion
                float x_tangential = 2.0 * p1 * x_normalized * y_normalized + p2 * (r2 + 2.0 * x_normalized * x_normalized);
                float y_tangential = p1 * (r2 + 2.0 * y_normalized * y_normalized) + 2.0 * p2 * x_normalized * y_normalized;

                // Distorted coordinates
                float x_distorted = x_radial + x_tangential;
                float y_distorted = y_radial + y_tangential;

                // Map back to pixel coordinates in the distorted image
                int distorted_x = (int)round(fx * x_distorted + cx);
                int distorted_y = (int)round(fy * y_distorted + cy);

                // Make sure coordinates are within image bounds
                if (distorted_x >= 0 && distorted_x < width && distorted_y >= 0 && distorted_y < height) {
                    int pixel_index = (distorted_y * width + distorted_x) * 3;
                    int buffer_index = (y * width + x) * 3;
                    buffer[buffer_index] = img[pixel_index];
                    buffer[buffer_index + 1] = img[pixel_index + 1];
                    buffer[buffer_index + 2] = img[pixel_index + 2];
                }
            }
        "#;

        let pro_que = ProQue::builder().src(src).dims((728, 544)).build()?;
        let buffer = vec![0u8; 728 * 544 * 3];

        let img_buffer = Buffer::<u8>::builder()
            .queue(pro_que.queue().clone())
            .flags(ocl::flags::MEM_READ_ONLY)
            .len(buffer.len())
            .copy_host_slice(&buffer)
            .build()?;

        let output_buffer = Buffer::<u8>::builder()
            .queue(pro_que.queue().clone())
            .flags(ocl::flags::MEM_WRITE_ONLY)
            .len(buffer.len())
            .build()?;

        Ok(Self {
            pro_que,
            img_buffer,
            output_buffer,
        })
    }

    pub fn undistort(&mut self, img: &DynamicImage) -> Result<RgbImage, ocl::Error> {
        // convert to RGB Vec<u8>
        let img = img.to_rgb8().into_raw().to_vec();
        let buffer = vec![0u8; 728 * 544 * 3];

        let t1 = std::time::Instant::now();
        self.img_buffer.write(&img).enq()?;
        self.output_buffer.write(&buffer).enq()?;
        println!("Buffer write time: {:?}", t1.elapsed());

        let kernel = self
            .pro_que
            .kernel_builder("undistort")
            .arg(&self.output_buffer)
            .arg(&self.img_buffer)
            // .arg(728i32)
            // .arg(544i32)
            .build()?;

        let t1 = std::time::Instant::now();
        unsafe {
            kernel.enq()?;
        }
        println!("Kernel execution time: {:?}", t1.elapsed());

        let t1 = std::time::Instant::now();
        let mut vec = vec![0u8; self.output_buffer.len()];
        self.output_buffer.read(&mut vec).enq()?;
        println!("Buffer read time: {:?}", t1.elapsed());

        // write the output buffer to a file
        let img = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(728, 544, vec).unwrap();
        Ok(img)
    }
}
