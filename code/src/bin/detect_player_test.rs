use image::imageops::FilterType;
use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::Rgba;
use matura::detect_player::PythonScript;
fn main() {
    let mut python_script = PythonScript::new();
    // let mut img = image::open("./player_training_images2/25_168_2_0305.png").unwrap();
    // let mut img = image::open("test_img.png").unwrap();
    let mut img = image::open("./recording_2024-11-14_17-57-43/0203.png").unwrap();
    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Calculate the width of each slice
    let slice_width = width / 7;

    // Crop the left-most slice
    let left_slice = img.crop(
        0,
        (1. / 3. * height as f32) as u32,
        slice_width,
        (1. / 3. * height as f32) as u32,
    );
    left_slice.save("left_slice.png").unwrap();
    let img = left_slice.resize_exact(128, 128, image::imageops::FilterType::Nearest);
    img.save("left_slice128.png").unwrap();
    // get player position
    let values = img
        .to_rgb8()
        .into_raw()
        .iter()
        .map(|&v| v as u8)
        .collect::<Vec<u8>>();
    assert_eq!(values.len(), 128 * 128 * 3);
    // println!("prep: {:?}", t.elapsed());
    let player_position = python_script.detect_player(&values);
    println!("Player position: {:?}", player_position);
}
