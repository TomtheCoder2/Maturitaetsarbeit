use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageBuffer, Rgba};
use image::Pixel;


fn main() {
    match read_image() {
        Ok(_) => println!("Image read successfully"),
        Err(e) => println!("Error reading image: {}", e),
    }
}

fn read_image() -> Result<(), Box<dyn std::error::Error>> {
    let t1 = std::time::Instant::now();
    let img = ImageReader::open("image2.png")?.decode()?;
    println!("Image read in {} ms", t1.elapsed().as_millis());
    println!("Image dimensions: {:?}", img.dimensions());
    let mut output = ImageBuffer::new(img.width(), img.height());

    let t1 = std::time::Instant::now();
    for pixel in img.pixels() {
        let (r, g, b, a) = pixel.2.channels4();
        // println!("Pixel at (x, y): ({}, {}) - RGBA: ({}, {}, {}, {})", pixel.0, pixel.1, r, g, b, a);
        // let a = (r as u32 + g as u32 + b as u32 / 3) as u8;
        // if r as i32 + g as i32 + b as i32 > 0 {
        //     output.put_pixel(pixel.0, pixel.1, Rgba([r, g, b, a]));
        if r as i32 + g as i32 > 250 && r > 100 && g > 100 && b < 100 {
            output.put_pixel(pixel.0, pixel.1, Rgba([r, g, b, a]));
        } else {
            output.put_pixel(pixel.0, pixel.1, Rgba([0, 0, 0, 0]));
        }
    }
    println!("Image processed in {} ms", t1.elapsed().as_millis());
    let t1 = std::time::Instant::now();
    output.save("output.png")?;
    println!("Image saved in {} ms", t1.elapsed().as_millis());
    Ok(())
}