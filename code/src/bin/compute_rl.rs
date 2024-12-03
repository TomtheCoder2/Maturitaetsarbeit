fn main() {
    // Path to the homography matrix CSV file
    let file_path = "./python_code_ma/homography_matrix.csv";

    // Read the homography matrix from the file
    let homography_matrix = matura::compute_rl_coords::read_homography_matrix(file_path);

    // Define the new point (550, 286) in pixel coordinates
    let mut new_pixel_point = (550.0, 286.0);

    let t0 = Instant::now();
    let n = 100;
    let mut real_world_point = (0.0, 0.0);
    for _ in 0..100 {
        // Apply the homography to the new point
        new_pixel_point.0 += 1.0;
        real_world_point = transform_point(homography_matrix, new_pixel_point);
    }
    let elapsed = t0.elapsed() / n;
    println!("Time elapsed: {:?}", elapsed);

    // Print the transformed real-world coordinates
    println!(
        "Real-world coordinates of the point (550, 286): ({}, {})",
        real_world_point.0, real_world_point.1
    );
}
