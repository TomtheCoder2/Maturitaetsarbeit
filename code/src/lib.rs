use image::RgbImage;
use rayon::prelude::*;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

pub mod arduino_com;
pub mod ball;
pub mod compute_rl_coords;
pub mod detect_player;
pub mod cam_thread;
pub mod live_feed;
pub mod plot;
pub mod image_utils;

/// Camera matrix and distortion coefficients from calibration
const CAMERA_MATRIX: [[f64; 3]; 3] = [
    [269.8211922992852, 0.0, 358.8342851000235],
    [0.0, 271.04841892629577, 279.672335523778],
    [0.0, 0.0, 1.0],
];
const DIST_COEFFS: [f64; 5] = [
    -0.10625421634791299,
    0.013677591305524796,
    2.405647979837061e-05,
    0.0023018687372285975,
    -0.0009689377710468434,
];

// const CAMERA_MATRIX: [[f64; 3]; 3] = [
//     [489.39739367, 0., 390.99969289],
//     [0., 493.83464995, 284.80547604],
//     [0., 0., 1.],
// ];

// const DIST_COEFFS: [f64; 5] = [
//     -0.01986707,
//     -0.62261306,
//     -0.00240047,
//     -0.02557275,
//     0.39225587,
// ];

// Define a new type that wraps the raw pointer
struct SafePtr(*mut u8);

// Implement Send and Sync for SafePtr
unsafe impl Send for SafePtr {}
unsafe impl Sync for SafePtr {}

/// Calculate the distorted coordinates given undistorted image coordinates.
fn distort_coords(x: f64, y: f64, fx: f64, fy: f64, cx: f64, cy: f64) -> (f64, f64) {
    // Distortion coefficients
    let k1 = DIST_COEFFS[0];
    let k2 = DIST_COEFFS[1];
    let p1 = DIST_COEFFS[2];
    let p2 = DIST_COEFFS[3];
    let k3 = DIST_COEFFS[4];

    // Normalize coordinates to [-1, 1]
    let x_normalized = (x - cx) / fx;
    let y_normalized = (y - cy) / fy;

    // Calculate radial distance
    let r2 = x_normalized * x_normalized + y_normalized * y_normalized;
    let r4 = r2 * r2;

    // Apply radial distortion
    let radial_distortion = 1.0 + k1 * r2 + k2 * r4 + k3 * r4 * r2;
    let x_radial = x_normalized * radial_distortion;
    let y_radial = y_normalized * radial_distortion;

    // Apply tangential distortion
    let x_tangential =
        2.0 * p1 * x_normalized * y_normalized + p2 * (r2 + 2.0 * x_normalized * x_normalized);
    let y_tangential =
        p1 * (r2 + 2.0 * y_normalized * y_normalized) + 2.0 * p2 * x_normalized * y_normalized;

    // Distorted coordinates
    let x_distorted = x_radial + x_tangential;
    let y_distorted = y_radial + y_tangential;

    // Map back to pixel coordinates
    let distorted_x = fx * x_distorted + cx;
    let distorted_y = fy * y_distorted + cy;

    (distorted_x, distorted_y)
}

/// Undistort an image and return a larger image that includes all original pixels.
pub fn undistort_image(img: &[u8], num_threads: u32, width: u32, height: u32) -> RgbImage {
    // let (width, height) = img.dimensions();

    // Camera matrix values
    let fx = CAMERA_MATRIX[0][0];
    let fy = CAMERA_MATRIX[1][1];
    let cx = CAMERA_MATRIX[0][2];
    let cy = CAMERA_MATRIX[1][2];

    // Step 1: Find the bounding box of the undistorted image by distorting the corners of the original image
    // let corners = [
    //     (0.0, 0.0),
    //     (width as f64 - 1.0, 0.0),
    //     (0.0, height as f64 - 1.0),
    //     (width as f64 - 1.0, height as f64 - 1.0),
    // ];

    // let mut MIN_X = f64::MAX;
    // let mut MAX_X = f64::MIN;
    // let mut MIN_Y = f64::MAX;
    // let mut MAX_Y = f64::MIN;

    // for &(x, y) in &corners {
    //     let (distorted_x, distorted_y) = distort_coords(x, y, fx, fy, cx, cy);
    //     MIN_X = MIN_X.min(distorted_x);
    //     MAX_X = MAX_X.max(distorted_x);
    //     MIN_Y = MIN_Y.min(distorted_y);
    //     MAX_Y = MAX_Y.max(distorted_y);
    // }
    let side_margin = 60i32;
    let top_margin = 10i32;
    let min_x = -side_margin;
    let max_x = width as i32 + side_margin;
    let min_y = -top_margin;
    let max_y = height as i32 + top_margin;

    // println!(
    //     "MIN_X: {}, MAX_X: {}, MIN_Y: {}, MAX_Y: {}",
    //     MIN_X, MAX_X, MIN_Y, MAX_Y
    // );

    // Step 2: Create the new undistorted image with a larger size
    let new_width = (max_x - min_x) as u32;
    let new_height = (max_y - min_y) as u32;
    println!("NEW_WIDTH: {}, NEW_HEIGHT: {}", new_width, new_height);

    let mut undistorted_img = RgbImage::new(new_width, new_height);
    let ptr = undistorted_img.as_mut_ptr();
    let safe_ptr = SafePtr(ptr);
    let safe_ptr = Arc::new(Mutex::new(safe_ptr));

    // Step 3: Process each pixel in parallel to undistort the image
    let block_size = (new_height as f32 / num_threads as f32).ceil() as u32;

    // let mut precomputation_table = Mutex::new(vec![None; (NEW_WIDTH * NEW_HEIGHT) as usize]);

    (0..num_threads).into_par_iter().for_each(|thread_n| {
        let start_y = thread_n * block_size;
        let end_y = (thread_n + 1) * block_size;

        let undistorted_block = unsafe {
            let safe_ptr = safe_ptr.lock().unwrap();
            safe_ptr
                .0
                .add((thread_n * block_size * new_width * 3) as usize)
        };

        for y in start_y..end_y.min(new_height) {
            let undistorted_row = unsafe {
                undistorted_block.add(((y - thread_n * block_size) * new_width * 3) as usize)
            };
            for x in 0..new_width {
                // Map the pixel back to the distorted image coordinates
                let original_x = x as f64 + min_x as f64;
                let original_y = y as f64 + min_y as f64;

                // Get the corresponding distorted coordinates
                let (distorted_x, distorted_y) =
                    distort_coords(original_x, original_y, fx, fy, cx, cy);

                let distorted_x = distorted_x.round() as i32;
                let distorted_y = distorted_y.round() as i32;

                // If the coordinates are within the bounds of the original image, copy the pixel
                if distorted_x >= 0
                    && distorted_x < width as i32
                    && distorted_y >= 0
                    && distorted_y < height as i32
                {
                    let index = ((distorted_y * width as i32 + distorted_x) * 3) as usize;
                    if index > img.len() {
                        println!(
                            "distorted_x: {}, distorted_y: {}, index: {} vs length: {}, ok: {}, width: {}, height: {}",
                            distorted_x,
                            distorted_y,
                            index,
                            img.len(),
                            index < img.len(),
                            width,
                            height
                        );
                    }
                    let pixel = [[img[index], img[index + 1], img[index + 2]]];
                    unsafe {
                        undistorted_row
                            .add((x as usize) * 3)
                            .copy_from_nonoverlapping(pixel[0].as_ptr(), 3);
                    }
                    // precomputation_table.lock().unwrap()[((y * NEW_WIDTH + x) as usize)] = Some(index);
                }
            }
        }
    });

    // let mut fi = File::create("./precomp_table.txt").unwrap();
    // fi.write_all(
    //     format!(
    //         "pub const UNDISTORT_MAP: [Option<usize>; {}] = [{}]",
    //         precomputation_table.lock().unwrap().len(),
    //         precomputation_table
    //             .lock()
    //             .unwrap()
    //             .clone()
    //             .into_iter()
    //             .map(|c| match c {
    //                 Some(c) => format!("Some({})", c),
    //                 None => "None".to_string(),
    //             })
    //             .collect::<Vec<_>>()
    //             .join(", ")
    //     )
    //     .as_bytes(),
    // )
    // .expect("Couldnt write to file");
    // println!("Saved to file");

    undistorted_img
}

use std::path::PathBuf;

pub fn increment_last_number_in_filename(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    let file_name = path.file_name()?.to_str()?;

    // Find the last number in the file name
    let mut last_number_start = None;
    let mut last_number_end = None;
    for (i, c) in file_name.char_indices().rev() {
        if c.is_ascii_digit() {
            if last_number_end.is_none() {
                last_number_end = Some(i + c.len_utf8());
            }
            last_number_start = Some(i);
        } else if last_number_end.is_some() {
            break;
        }
    }

    if let (Some(start), Some(end)) = (last_number_start, last_number_end) {
        let number_str = &file_name[start..end];
        if let Ok(number) = number_str.parse::<u32>() {
            let incremented_number = number + 1;
            let new_file_name = format!(
                "{}{}{}",
                &file_name[..start],
                incremented_number,
                &file_name[end..]
            );
            let mut new_path = PathBuf::from(path.parent()?);
            new_path.push(new_file_name);
            return Some(new_path.to_string_lossy().to_string());
        }
    }

    None
}

/// Generate precomputation table for undistortion.
pub fn gen_table(
    original_width: u32,
    original_height: u32,
    new_width: u32,
    new_height: u32,
    x_offset: i32,
    y_offset: i32,
) -> Vec<i32> {
    // Camera matrix values
    let fx = CAMERA_MATRIX[0][0];
    let fy = CAMERA_MATRIX[1][1];
    let cx = CAMERA_MATRIX[0][2];
    let cy = CAMERA_MATRIX[1][2];

    let mut precomputation_table = vec![];

    for y in 0..new_height {
        for x in 0..new_width {
            let x = x as i32 + x_offset;
            let y = y as i32 + y_offset;
            // Map the pixel back to the distorted image coordinates
            let (distorted_x, distorted_y) = distort_coords(x as f64, y as f64, fx, fy, cx, cy);

            let distorted_x = distorted_x.round() as i32;
            let distorted_y = distorted_y.round() as i32;

            // If the coordinates are within the bounds of the original image, map the pixel
            let index = if distorted_x >= 0
                && distorted_x < original_width as i32
                && distorted_y >= 0
                && distorted_y < original_height as i32
            {
                let index = ((distorted_y * original_width as i32 + distorted_x) * 3) as usize;
                index as i32
            } else {
                -1 // black pixel (outside the image bounds)
            };
            precomputation_table.push(index);
        }
    }
    precomputation_table
}

/// Undistort an image using the precomputed table.
pub fn undistort_image_table(
    img: &[u8],
    undistorted_img: &mut [u8],
    table: &[i32],
    new_width: u32,
    new_height: u32,
) {
    // Assert that the image has the correct size
    assert_eq!(
        undistorted_img.len(),
        3 * new_width as usize * new_height as usize
    );

    for i in 0..new_height * new_width {
        let index = table[i as usize];

        if index != -1 {
            #[inline]
            /// Helper function to avoid code duplication
            fn set_pixel(undistorted_img: &mut [u8], img: &[u8], pixel_index: usize, i: usize) {
                undistorted_img[i] = img[pixel_index];
            }
            let pixel_index = index as usize;
            set_pixel(undistorted_img, img, pixel_index, i as usize * 3);
            set_pixel(undistorted_img, img, pixel_index + 1, i as usize * 3 + 1);
            set_pixel(undistorted_img, img, pixel_index + 2, i as usize * 3 + 2);
        }
    }
}

pub fn undistort_image_table_old(img: &[u8], table: &[(usize, i32)]) -> RgbImage {
    let width = 728;
    let height = 544;

    let min_x = -1200f64 + width as f64;
    let max_x = 1200f64;
    let min_y = -1200f64 + height as f64;
    let max_y = 1200f64;

    let new_width = (max_x - min_x).ceil() as u32;
    let new_height = (max_y - min_y).ceil() as u32;

    let mut undistorted_img = RgbImage::new(new_width, new_height);
    let mut start = true;
    let mut counter = 0;
    let mut map_index = 0;
    let mut image_index = 0;
    for i in 0..new_height * new_width {
        let index = if counter == 0 {
            map_index += if start {
                start = false;
                0
            } else {
                1
            };
            let map_info = match table.get(map_index) {
                Some(map_info) => map_info,
                None => {
                    println!(
                        "Index out of bounds: {}/{}, i: {}/{}",
                        map_index,
                        table.len(),
                        i,
                        new_height * new_width
                    );
                    break;
                }
            };
            counter = map_info.0 + 1;
            image_index = map_info.1;
            image_index
        } else {
            counter -= 1;
            image_index
        };
        if index != -1 {
            // let index = index *3;
            let x = i % new_width;
            let y = i / new_width;
            let index = index as usize;
            let pixel = [img[index], img[index + 1], img[index + 2]];
            undistorted_img.put_pixel(x, y, image::Rgb(pixel));
        }
    }

    undistorted_img
}

/// Undistort an image and return a larger image that includes all original pixels.
pub fn gen_table_old(width: u32, height: u32) -> Vec<(usize, i32)> {
    // let (width, height) = img.dimensions();

    // Camera matrix values
    let fx = CAMERA_MATRIX[0][0];
    let fy = CAMERA_MATRIX[1][1];
    let cx = CAMERA_MATRIX[0][2];
    let cy = CAMERA_MATRIX[1][2];

    let min_x = -1200f64 + width as f64;
    let max_x = 1200f64;
    let min_y = -1200f64 + height as f64;
    let max_y = 1200f64;

    // println!(
    //     "MIN_X: {}, MAX_X: {}, MIN_Y: {}, MAX_Y: {}",
    //     MIN_X, MAX_X, MIN_Y, MAX_Y
    // );

    // Step 2: Create the new undistorted image with a larger size
    let new_width = (max_x - min_x).ceil() as u32;
    let new_height = (max_y - min_y).ceil() as u32;

    // we make an encoding where the first numbers says how many times the second number is repeated
    let mut precomputation_table = vec![];

    let mut last_number = -2;
    let mut same_counter = 0;
    for y in 0..new_height {
        for x in 0..new_width {
            // Map the pixel back to the distorted image coordinates
            let original_x = x as f64 + min_x;
            let original_y = y as f64 + min_y;

            // Get the corresponding distorted coordinates
            let (distorted_x, distorted_y) = distort_coords(original_x, original_y, fx, fy, cx, cy);

            let distorted_x = distorted_x.round() as i32;
            let distorted_y = distorted_y.round() as i32;

            // If the coordinates are within the bounds of the original image, copy the pixel
            let index = if distorted_x >= 0
                && distorted_x < width as i32
                && distorted_y >= 0
                && distorted_y < height as i32
            {
                let index = ((distorted_y * width as i32 + distorted_x) * 3) as usize;
                index as i32
            } else {
                -1
            };

            if index == last_number {
                // println!("index: {}, last_number: {}", index, last_number);
                same_counter += 1;
            } else {
                if same_counter > 0 {
                    precomputation_table.push((same_counter, last_number));
                    // println!("pushing: ({}, {})", same_counter, last_number);
                }
                last_number = index;
                same_counter = 1;
            }
        }
    }

    precomputation_table.push((same_counter, last_number));
    // precomputation_table.remove(0);
    // println!("table: {:?}", precomputation_table);

    // save to file for debugging
    // let mut file = File::create("precomputation_table.txt").unwrap();
    // for (count, index) in precomputation_table.iter() {
    //     writeln!(file, "{}, {}", count, index).unwrap();
    // }

    println!(
        "Length of precomputation table: {}",
        precomputation_table.len()
    );

    precomputation_table
}
