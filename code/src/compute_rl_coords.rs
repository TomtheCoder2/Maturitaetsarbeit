use nalgebra::{Matrix3, Vector3};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct RLCompute {
    homography_matrix: Matrix3<f32>,
}

impl RLCompute {
    pub fn new() -> Self {
        RLCompute::new_f("./python_code_ma/homography_matrix.csv")
    }

    pub fn new_f(file_path: &str) -> Self {
        RLCompute {
            homography_matrix: read_homography_matrix(file_path),
        }
    }

    // Function to apply homography to a new pixel point using nalgebra
    pub fn transform_point(&self, pixel_point: (f32, f32)) -> (f32, f32) {
        let pixel_vector = Vector3::new(pixel_point.0, pixel_point.1, 1.0);

        // Perform the matrix multiplication
        let real_world_vector = self.homography_matrix * pixel_vector;

        // Normalize by dividing by the third coordinate
        let real_world_x = real_world_vector.x / real_world_vector.z;
        let real_world_y = real_world_vector.y / real_world_vector.z;

        (real_world_x, real_world_y)
    }
}

// Function to read homography matrix from CSV file
pub fn read_homography_matrix(file_path: &str) -> Matrix3<f32> {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut matrix_data = vec![];
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let values: Vec<f32> = line
            .split(',')
            .map(|s| s.trim().parse().expect("Unable to parse value"))
            .collect();
        matrix_data.extend(values);
    }

    // Build the homography matrix (3x3) from the file data
    Matrix3::new(
        matrix_data[0],
        matrix_data[1],
        matrix_data[2],
        matrix_data[3],
        matrix_data[4],
        matrix_data[5],
        matrix_data[6],
        matrix_data[7],
        matrix_data[8],
    )
}

// Function to apply homography to a new pixel point using nalgebra
pub fn transform_point(h_matrix: Matrix3<f32>, pixel_point: (f32, f32)) -> (f32, f32) {
    let pixel_vector = Vector3::new(pixel_point.0, pixel_point.1, 1.0);

    // Perform the matrix multiplication
    let real_world_vector = h_matrix * pixel_vector;

    // Normalize by dividing by the third coordinate
    let real_world_x = real_world_vector.x / real_world_vector.z;
    let real_world_y = real_world_vector.y / real_world_vector.z;

    (real_world_x, real_world_y)
}
