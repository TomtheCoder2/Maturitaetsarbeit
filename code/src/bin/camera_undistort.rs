use image::GenericImageView;
use image::Pixel;
use image::{DynamicImage, RgbImage};
use image::{ImageBuffer, Rgb};
use nalgebra::{Matrix3, Vector2};
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

// Camera matrix and distortion coefficients from calibration
const CAMERA_MATRIX: [[f64; 3]; 3] = [
    [489.39739367, 0., 390.99969289],
    [0., 493.83464995, 284.80547604],
    [0., 0., 1.],
];

const DIST_COEFFS: [f64; 5] = [
    -0.01986707,
    -0.62261306,
    -0.00240047,
    -0.02557275,
    0.39225587,
];

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
    let number = 7;
    let mut img = image::open(format!("input_images/input_image{}.png", number))
        .expect("Failed to open image");
    let img = img.to_rgb8();

    let width = img.width();
    let height = img.height();
    let side_margin = 70i32;
    let top_margin = 10i32;
    let min_x = -side_margin;
    let max_x = width as i32 + side_margin;
    let min_y = -top_margin;
    let max_y = height as i32 + top_margin;
    let new_width = (max_x - min_x) as u32;
    let new_height = (max_y - min_y) as u32;
    println!("old width: {}, old height: {}", width, height);
    println!("new width: {}, new height: {}", new_width, new_height);
    let precompute = matura::gen_table(width, height, new_width, new_height, min_x, min_y);

    // matura::gen_table(&img, 30, 728, 544);
    //     // let image = matura::undistort_image(&img, 30, width, height);
    let n = 1000;
    let mut image = vec![0u8; (new_width * new_height * 3) as usize];
    let t1 = Instant::now();
    for _ in 0..n {
        matura::undistort_image_table(&img, &mut image, &precompute, new_width, new_height);
    }
    let dt = t1.elapsed();
    println!(
        "Time taken for one avg call: {:?}, time per pixel: {:?}",
        dt / n,
        dt / n / (new_width * new_height)
    );
    // let image_original = undistort_image_original(&img, camera_matrix, dist_coeffs);
    // assert_eq!(image, image_original);
    // Save undistorted image
    let mut final_image = RgbImage::new(new_width, new_height);
    for x in 0..new_width {
        for y in 0..new_height {
            let pixel_index = (y * new_width + x) as usize * 3;
            let pixel = &image[pixel_index..pixel_index + 3];
            final_image.put_pixel(x, y, Rgb([pixel[0], pixel[1], pixel[2]]));
        }
    }
    final_image
        .save(format!("output_images/output{}.png", number))
        .expect("Failed to save image");

    // Load image
    // let mut img = image::open("input_image.png").expect("Failed to open image");
    // let mut img = img.to_rgb8();

    // // Undistort image
    // let t1 = std::time::Instant::now();
    // for i in 0..10 {
    //     let image = undistort_image2(&img, camera_matrix.into(), dist_coeffs, new_camera_matrix);
    //     img = image::DynamicImage::ImageRgb8(image).into();
    // }
    // println!("Time taken 2: {:?}", t1.elapsed() / 10);

    // Load image
    // let mut img = image::open("input_image2.png").expect("Failed to open image");
    println!("Image size: {:?}", img.dimensions());
    // 396032 pixels => 0.396 MP

    let max_threads = 100; // Adjust this based on your system's capabilities
                           // Undistort image
    let best_thread_count = (10..=max_threads)
        .map(|num_threads| {
            let t1 = std::time::Instant::now();
            for i in 0..10 {
                // let image = matura::undistort_image(&img, num_threads, img.width(), img.height());
                // img = image::DynamicImage::ImageRgb8(image).into();
            }
            let duration = t1.elapsed() / 10;
            // println!(
            // "Time taken original: {:?} with {} threads",
            // duration, num_threads
            // );
            (num_threads, duration)
        })
        .min_by_key(|&(_, duration)| duration)
        .unwrap();
    println!(
        "Optimal number of threads: {} with duration: {:?}",
        best_thread_count.0, best_thread_count.1
    );
}
