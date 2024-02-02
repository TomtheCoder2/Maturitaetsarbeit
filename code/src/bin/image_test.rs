use std::fmt::{Display, Formatter};
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
    let args: Vec<String> = std::env::args().collect();
    // check first arg
    let file_name = if args.len() < 2 {
        println!("Usage: {} <image>", args[0]);
        "frame2.jpg"
        // return Ok(());
    } else {
        &args[1]
    };
    let t1 = std::time::Instant::now();
    let img = ImageReader::open(file_name)?.decode()?;
    const SCALING: u32 = 4;
    let new_width = img.width() / SCALING;
    let new_height = img.height() / SCALING;
    // let mut output = ImageBuffer::new(img.width() / SCALING + 1, img.height() / SCALING + 1);
    let mut output = ImageBuffer::new(img.width(), img.height());
    output.put_pixel(0u32, 0u32, Rgba([0u8, 0, 0, 0]));
    // ImageBuffer::new((img.width() as f32 * scaling_factor) as u32, (img.height() as f32 * scaling_factor) as u32);

    #[derive(Debug)]
    struct BallRaw {
        sum_x: u64,
        sum_y: u64,
        count: u64,
        max_x: u32,
        max_y: u32,
        min_x: u32,
        min_y: u32,
    }
    // let mut new_output = ImageBuffer::new(img.width() / SCALING + 1, img.height() / SCALING + 1);
    let mut balls_raw: Vec<BallRaw> = Vec::new();
    // let pixels = img.pixels().collect::<Vec<_>>();
    println!("Image read in {} ms", t1.elapsed().as_millis());
    println!("Image dimensions: {:?}", img.dimensions());
    let t1 = std::time::Instant::now();
    let mut counter = 0;
    // println!("pixels: {}", img.pixels().count());
    for (_i, pixel) in img.pixels().enumerate() {
        if counter == SCALING - 1 {
            // let pixel = pixels[i];
            // println!("pixel: {:?}", pixel);
            let rgb = pixel.2.channels();
            let (r, g, b, a) = (rgb[0], rgb[1], rgb[2], rgb[3]);
            let x = pixel.0 / SCALING;
            let y = pixel.1 / SCALING;
            // filter out the not yellow pixels
            if r as i32 + g as i32 > 250 && r > 100 && g > 100 && b < 100 {
                // output.put_pixel(x, y, Rgba([r, g, b, a]));
                // go through all balls, check if the pixel is close to any of them (within 1 pixels)
                let mut found = false;
                for ball in balls_raw.iter_mut() {
                    const MAX_DISTANCE: u32 = 1;
                    if x as i32 >= ball.min_x as i32 - MAX_DISTANCE as i32 &&
                        x <= ball.max_x + MAX_DISTANCE &&
                        y as i32 >= ball.min_y as i32 - MAX_DISTANCE as i32 &&
                        y <= ball.max_y + MAX_DISTANCE {
                        ball.sum_x += x as u64;
                        ball.sum_y += y as u64;
                        ball.count += 1;
                        if x < ball.min_x {
                            ball.min_x = x;
                        }
                        if x > ball.max_x {
                            ball.max_x = x;
                        }
                        if y < ball.min_y {
                            ball.min_y = y;
                        }
                        if y > ball.max_y {
                            ball.max_y = y;
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    balls_raw.push(BallRaw {
                        sum_x: x as u64,
                        sum_y: y as u64,
                        count: 1,
                        max_x: x,
                        max_y: y,
                        min_x: x,
                        min_y: y,
                    });
                }
            } else {
                // output.put_pixel(x, y, Rgba([r, g, b, a / 2]));
            }
            counter = 0;
        } else {
            counter += 1;
        }
    }
    // filter out the balls that are too small
    balls_raw.retain(|ball| ball.count > 2);
    println!("Image processed in {:?}", t1.elapsed());
    // println!("balls: {:?}, len: {}", balls, balls.len());
    struct Ball {
        x: u32,
        y: u32,
        radius: f32,
    }
    impl Display for Ball {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Ball: ({}, {}), radius: {}", self.x, self.y, self.radius)
        }
    }
    let mut balls = Vec::new();
    for (i, pixel) in img.pixels().enumerate() {
        let rgb = pixel.2.channels();
        let (r, g, b, a) = (rgb[0], rgb[1], rgb[2], rgb[3]);
        let x = pixel.0;
        let y = pixel.1;
        output.put_pixel(x, y, Rgba([r, g, b, a / 2]));
    }
    // crate lines with the centers of the balls that are between 25% and 75% of the image
    for ball in balls_raw.iter() {
        let x = (ball.sum_x / ball.count) as u32;
        let y = (ball.sum_y / ball.count) as u32;
        if x > new_width / 3 && x < new_width * 3 / 4 && y > new_height / 3 && y < new_height * 3 / 4 {
            // todo: check if this is even right
            let radius = (((ball.max_x - ball.min_x).pow(2) + (ball.max_y - ball.min_y).pow(2)) as f32).sqrt() / 2.0;
            for i in 0..output.width() {
                // dont show the lines at radius*2
                if (i as i32 - (x * SCALING) as i32).abs() < (radius * SCALING as f32) as i32 * 3 {
                    continue;
                }
                output.put_pixel(i, y * SCALING, Rgba([255, 0, 0, 255]));
            }
            for i in 0..output.height() {
                if (i as i32 - (y * SCALING) as i32).abs() < (radius * SCALING as f32) as i32 * 3 {
                    continue;
                }
                output.put_pixel(x * SCALING, i, Rgba([255, 0, 0, 255]));
            }
            // make a circle around the ball
            for i in 0..360 {
                let x = ((x * SCALING) as f32 + radius * SCALING as f32 * (i as f32).to_radians().cos()) as u32;
                let y = ((y * SCALING) as f32 + radius * SCALING as f32 * (i as f32).to_radians().sin()) as u32;
                output.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            }
            println!("center: ({}, {}) normal coords: ({}, {})", x, y, x * SCALING, y * SCALING);
            balls.push(Ball {
                x: x * SCALING,
                y: y * SCALING,
                radius: radius * SCALING as f32,
            });
        }
    }
    println!("Image filtered in {} ms", t1.elapsed().as_millis());
    let t1 = std::time::Instant::now();
    output.save("output.png")?;
    println!("Image saved in {} ms", t1.elapsed().as_millis());
    // print all the balls
    for ball in balls.iter() {
        println!("{}", ball);
    }
    Ok(())
}