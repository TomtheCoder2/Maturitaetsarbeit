use image::{DynamicImage, GenericImage, GenericImageView, ImageError};
use std::fs;
use std::path::Path;

fn main() -> Result<(), ImageError> {
    let input_folder = "./recording_2024-11-10_10-50-58";
    let output_folder = "./output_slices/";

    // Create the output folder if it doesn't exist
    fs::create_dir_all(output_folder)?;

    // Iterate over all files in the input folder
    for entry in fs::read_dir(input_folder)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file and has an image extension
        if path.is_file()
            && path
                .extension()
                .map_or(false, |ext| ext == "jpg" || ext == "png")
        {
            // Load the image
            let mut img: DynamicImage = image::open(&path)?;

            // Get the dimensions of the image
            let (width, height) = img.dimensions();

            // Calculate the width of each slice
            let slice_width = width / 7;

            // Crop the left-most slice
            let mut left_slice = img.crop(
                0,
                (1. / 3. * height as f32) as u32,
                slice_width,
                (1. / 3. * height as f32) as u32,
            );
            for x in 0..left_slice.width() {
                for y in 0..left_slice.height() {
                    let pixel = left_slice.get_pixel(x, y);
                    let blue = pixel[2];
                    // left_slice.put_pixel(x, y, image::Rgba([blue, blue, blue, 255]));
                }
            }

            // Create the output path
            let output_path = Path::new(output_folder).join(Path::new(&format!(
                "2_{}",
                path.file_name().unwrap().to_str().unwrap()
            )));

            // Save the left-most slice to the output folder
            left_slice.save(output_path)?;
        }
    }

    Ok(())
}
