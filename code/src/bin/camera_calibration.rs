extern crate opencv;
use opencv::{
    calib3d::{calibrate_camera, corner_sub_pix, draw_chessboard_corners, find_chessboard_corners},
    core::{Mat, MatTraitConst, Point2f, Size, TermCriteria, TermCriteria_Type},
    highgui,
    imgcodecs::imread,
    imgproc::{cvt_color, good_features_to_track, COLOR_BGR2GRAY},
    prelude::*,
    types::VectorOfMat,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Chessboard size (number of internal corners)
    let pattern_size = Size::new(9, 6);
    let square_size = 1.8625; // Size of a square in your defined unit (e.g., meters) , for me its cm

    // Arrays to store object points and image points
    let mut object_points = VectorOfMat::new();
    let mut image_points = VectorOfMat::new();

    // Prepare the object points, like (0,0,0), (1,0,0), (2,0,0) ... , based on the square size
    let mut objp = Mat::zeros(
        pattern_size.height,
        pattern_size.width,
        opencv::core::CV_32FC3,
    )?
    .to_vec_2d::<Point2f>()?;
    for i in 0..pattern_size.height {
        for j in 0..pattern_size.width {
            objp[i as usize][j as usize] =
                Point2f::new(j as f32 * square_size, i as f32 * square_size);
        }
    }

    // Loop through each calibration image
    let mut images = vec!["image1.jpg", "image2.jpg", "image3.jpg"]; // Add your calibration images here
    for image_path in &images {
        let img = imread(image_path, 0)?;
        let mut gray = Mat::default()?;
        cvt_color(&img, &mut gray, COLOR_BGR2GRAY, 0)?;

        let mut corners = Mat::default()?;
        let found = find_chessboard_corners(&gray, pattern_size, &mut corners, 0)?;

        if found {
            // Refine corner locations
            corner_sub_pix(
                &gray,
                &mut corners,
                Size::new(11, 11),
                Size::new(-1, -1),
                TermCriteria::new(TermCriteria_Type::COUNT + TermCriteria_Type::EPS, 30, 0.1)?,
            )?;

            // Store the object points and image points
            let objp_mat = Mat::from_slice(&objp)?;
            object_points.push(objp_mat);
            image_points.push(corners);

            // Draw and display the corners
            draw_chessboard_corners(&img, pattern_size, &corners, found)?;
            highgui::imshow("Corners", &img)?;
            highgui::wait_key(0)?;
        }
    }

    // Camera matrix and distortion coefficients
    let mut camera_matrix = Mat::eye(3, 3, opencv::core::CV_64F)?;
    let mut dist_coeffs = Mat::zeros(8, 1, opencv::core::CV_64F)?;

    let mut rvecs = VectorOfMat::new();
    let mut tvecs = VectorOfMat::new();

    // Perform camera calibration
    calibrate_camera(
        &object_points,
        &image_points,
        pattern_size,
        &mut camera_matrix,
        &mut dist_coeffs,
        &mut rvecs,
        &mut tvecs,
        0,
    )?;

    // Print the camera matrix and distortion coefficients
    println!("Camera Matrix: {:?}", camera_matrix);
    println!("Distortion Coefficients: {:?}", dist_coeffs);

    Ok(())
}
