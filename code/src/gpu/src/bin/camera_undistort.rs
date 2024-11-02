use gpu::*;
fn main() {
    let camera_matrix = CAMERA_MATRIX;
    let dist_coeffs = DIST_COEFFS;
    let mut img = image::open("input_image.png").expect("Failed to open image");
    let mut distortion_correction = DistortionCorrection::new().unwrap();
    let image = distortion_correction.undistort(&img).unwrap();
    image.save("output_image.png").unwrap();
    // let mut img = img.to_rgb8();

    let t1 = std::time::Instant::now();
    for i in 0..10 {
        let image = distortion_correction.undistort(&img).unwrap();
        img = image::DynamicImage::ImageRgb8(image).into();
    }
    let duration = t1.elapsed() / 10;
    println!("Time taken gpu: {:?}", duration);
}
