use image::GenericImageView;
use image::Pixel;
use image::{DynamicImage, RgbImage};
use image::{ImageBuffer, Rgb};
use nalgebra::{Matrix3, Vector2};
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

// Camera matrix and distortion coefficients from calibration
const CAMERA_MATRIX: [[f64; 3]; 3] = [
    [458.11615111, 0.0, 367.97913048],
    [0.0, 458.19122834, 272.04155943],
    [0.0, 0.0, 1.0],
];
const DIST_COEFFS: [f64; 5] = [-0.32281387, 0.17342487, -0.0012062, 0.004915, -0.07841724];

fn undistort_image_original(
    img: &DynamicImage,
    camera_matrix: [[f64; 3]; 3],
    dist_coeffs: [f64; 5],
) -> RgbImage {
    let (width, height) = img.dimensions();
    let mut undistorted_img = RgbImage::new(width, height);

    // Camera matrix values
    let fx = camera_matrix[0][0] as f64;
    let fy = camera_matrix[1][1] as f64;
    let cx = camera_matrix[0][2] as f64;
    let cy = camera_matrix[1][2] as f64;

    // Distortion coefficients
    let k1 = dist_coeffs[0];
    let k2 = dist_coeffs[1];
    let p1 = dist_coeffs[2];
    let p2 = dist_coeffs[3];
    let k3 = dist_coeffs[4];

    for y in 0..height {
        for x in 0..width {
            // Normalize coordinates to [-1, 1]
            let x_normalized = (x as f64 - cx) / fx;
            let y_normalized = (y as f64 - cy) / fy;

            // Calculate radial distance
            let r2 = x_normalized * x_normalized + y_normalized * y_normalized;

            // Apply radial distortion
            let radial_distortion = 1.0 + k1 * r2 + k2 * r2 * r2 + k3 * r2 * r2 * r2;
            let x_radial = x_normalized * radial_distortion;
            let y_radial = y_normalized * radial_distortion;

            // Apply tangential distortion
            let x_tangential = 2.0 * p1 * x_normalized * y_normalized
                + p2 * (r2 + 2.0 * x_normalized * x_normalized);
            let y_tangential = p1 * (r2 + 2.0 * y_normalized * y_normalized)
                + 2.0 * p2 * x_normalized * y_normalized;

            // Distorted coordinates
            let x_distorted = x_radial + x_tangential;
            let y_distorted = y_radial + y_tangential;

            // Map back to pixel coordinates in the distorted image
            let distorted_x = (fx * x_distorted + cx).round() as i32;
            let distorted_y = (fy * y_distorted + cy).round() as i32;

            // Make sure coordinates are within image bounds
            if distorted_x >= 0
                && distorted_x < width as i32
                && distorted_y >= 0
                && distorted_y < height as i32
            {
                let pixel = img.get_pixel(distorted_x as u32, distorted_y as u32);
                undistorted_img.put_pixel(x, y, pixel.to_rgb());
            }
        }
    }

    undistorted_img
}

// Define a new type that wraps the raw pointer
struct SafePtr(*mut u8);

// Implement Send and Sync for SafePtr
unsafe impl Send for SafePtr {}
unsafe impl Sync for SafePtr {}

/// Undistort an image using the camera matrix and distortion coefficients
/// time: O(n^2), 16.1094ms
fn undistort_image(img: &DynamicImage, num_threads: u32) -> RgbImage {
    let (width, height) = img.dimensions();
    let mut undistorted_img = RgbImage::new(width, height);
    let ptr = undistorted_img.as_mut_ptr();
    let safe_ptr = SafePtr(ptr);
    let safe_ptr = Arc::new(Mutex::new(safe_ptr));

    // Camera matrix values
    let fx = CAMERA_MATRIX[0][0] as f64;
    let fy = CAMERA_MATRIX[1][1] as f64;
    let cx = CAMERA_MATRIX[0][2] as f64;
    let cy = CAMERA_MATRIX[1][2] as f64;

    // Distortion coefficients
    let k1 = DIST_COEFFS[0];
    let k2 = DIST_COEFFS[1];
    let p1 = DIST_COEFFS[2];
    let p2 = DIST_COEFFS[3];
    let k3 = DIST_COEFFS[4];
    // let num_threads = 16;
    let block_size = (height as f32 / num_threads as f32).ceil() as u32;

    // for y in 0..height {
    (0..num_threads).into_par_iter().for_each(|thread_n| {
        let undistorted_block = unsafe {
            let safe_ptr = safe_ptr.lock().unwrap();
            safe_ptr.0.add((thread_n * block_size * width * 3) as usize)
        };
        for y in (thread_n * block_size)..((thread_n + 1) * block_size) {
            if y >= height {
                break;
            }
            let undistorted_row = unsafe {
                undistorted_block.add(((y - thread_n * block_size) * width * 3) as usize)
            };
            for x in 0..width {
                // Normalize coordinates to [-1, 1]
                let x_normalized = (x as f64 - cx) / fx;
                let y_normalized = (y as f64 - cy) / fy;

                // Calculate radial distance
                let r2 = x_normalized * x_normalized + y_normalized * y_normalized;
                let r4 = r2 * r2;

                // Apply radial distortion
                let radial_distortion = 1.0 + k1 * r2 + k2 * r4 + k3 * r4 * r2;
                let x_radial = x_normalized * radial_distortion;
                let y_radial = y_normalized * radial_distortion;

                // Apply tangential distortion
                let x_tangential = 2.0 * p1 * x_normalized * y_normalized
                    + p2 * (r2 + 2.0 * x_normalized * x_normalized);
                let y_tangential = p1 * (r2 + 2.0 * y_normalized * y_normalized)
                    + 2.0 * p2 * x_normalized * y_normalized;

                // Distorted coordinates
                let x_distorted = x_radial + x_tangential;
                let y_distorted = y_radial + y_tangential;

                // Map back to pixel coordinates in the distorted image
                let distorted_x = (fx * x_distorted + cx).round() as i32;
                let distorted_y = (fy * y_distorted + cy).round() as i32;

                // Make sure coordinates are within image bounds
                if distorted_x >= 0
                    && distorted_x < width as i32
                    && distorted_y >= 0
                    && distorted_y < height as i32
                {
                    let pixel = img.get_pixel(distorted_x as u32, distorted_y as u32);
                    unsafe {
                        undistorted_row
                            .add((x as usize) * 3)
                            .copy_from_nonoverlapping(pixel.to_rgb().0.as_ptr(), 3);
                    }
                }
            }
        }
    });

    undistorted_img
}

fn undistort_image2(
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    camera_matrix: Matrix3<f64>,
    dist_coeffs: [f64; 5],
    new_camera_matrix: Matrix3<f64>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = (image.width() as usize, image.height() as usize);
    let mut undistorted_image = ImageBuffer::new(image.width(), image.height());

    let inv_camera_matrix = camera_matrix.try_inverse().unwrap();
    let new_camera_matrix_arc = Arc::new(new_camera_matrix);
    let inv_camera_matrix_arc = Arc::new(inv_camera_matrix);

    undistorted_image
        .enumerate_pixels_mut()
        .par_bridge() // Parallelize over pixels using rayon
        .for_each(|(x, y, pixel)| {
            let new_camera_matrix = new_camera_matrix_arc.clone();
            let inv_camera_matrix = inv_camera_matrix_arc.clone();

            // Normalize coordinates to [-1, 1]
            let u = (x as f64 - new_camera_matrix[(0, 2)]) / new_camera_matrix[(0, 0)];
            let v = (y as f64 - new_camera_matrix[(1, 2)]) / new_camera_matrix[(1, 1)];

            let r2 = u * u + v * v;
            let r4 = r2 * r2;
            let r6 = r4 * r2;

            // Apply radial distortion
            let radial = 1.0 + dist_coeffs[0] * r2 + dist_coeffs[1] * r4 + dist_coeffs[4] * r6;

            // Apply tangential distortion
            let delta_u = 2.0 * dist_coeffs[2] * u * v + dist_coeffs[3] * (r2 + 2.0 * u * u);
            let delta_v = dist_coeffs[2] * (r2 + 2.0 * v * v) + 2.0 * dist_coeffs[3] * u * v;

            let undistorted_u =
                (u * radial + delta_u) * inv_camera_matrix[(0, 0)] + inv_camera_matrix[(0, 2)];
            let undistorted_v =
                (v * radial + delta_v) * inv_camera_matrix[(1, 1)] + inv_camera_matrix[(1, 2)];

            let src_x = undistorted_u.round() as i32;
            let src_y = undistorted_v.round() as i32;

            if src_x >= 0 && src_x < width as i32 && src_y >= 0 && src_y < height as i32 {
                let original_pixel = image.get_pixel(src_x as u32, src_y as u32);
                *pixel = *original_pixel;
            }
        });

    undistorted_image
}

fn main() {
    let camera_matrix = CAMERA_MATRIX;
    let dist_coeffs = DIST_COEFFS;
    let new_camera_matrix = Matrix3::new(1000.0, 0.0, 640.0, 0.0, 1000.0, 360.0, 0.0, 0.0, 1.0);
    let img = image::open("input_image.png").expect("Failed to open image");

    let image = undistort_image(&img, 16);
    let image_original = undistort_image_original(&img, camera_matrix, dist_coeffs);
    assert_eq!(image, image_original);
    // Save undistorted image
    image
        .save("output_image.png")
        .expect("Failed to save image");

    // Load image
    let mut img = image::open("input_image.png").expect("Failed to open image");
    let mut img = img.to_rgb8();

    // Undistort image
    let t1 = std::time::Instant::now();
    for i in 0..10 {
        let image = undistort_image2(&img, camera_matrix.into(), dist_coeffs, new_camera_matrix);
        img = image::DynamicImage::ImageRgb8(image).into();
    }
    println!("Time taken 2: {:?}", t1.elapsed() / 10);

    // Load image
    let mut img = image::open("input_image.png").expect("Failed to open image");
    println!("Image size: {:?}", img.dimensions());
    // 396032 pixels => 0.396 MP

    let max_threads = 100; // Adjust this based on your system's capabilities
                           // Undistort image
    let best_thread_count = (1..=max_threads)
        .map(|num_threads| {
            let t1 = std::time::Instant::now();
            for i in 0..10 {
                let image = undistort_image(&img, num_threads);
                img = image::DynamicImage::ImageRgb8(image).into();
            }
            let duration = t1.elapsed() / 10;
            println!(
                "Time taken original: {:?} with {} threads",
                duration, num_threads
            );
            (num_threads, duration)
        })
        .min_by_key(|&(_, duration)| duration)
        .unwrap();
    println!(
        "Optimal number of threads: {} with duration: {:?}",
        best_thread_count.0, best_thread_count.1
    );
}
