use image::io::Reader as ImageReader;
use image::Pixel;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer};
use image::{RgbImage, Rgba};
use matura::ball::Vec2;
use matura::debug;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;

pub fn subtract_image(original: &mut [u8], other: &[u8]) {
    assert_eq!(original.len(), other.len());
    for i in 0..original.len() {
        // todo avoid casting
        original[i] = (original[i] as i32 - other[i] as i32).max(0).min(255) as u8;
        // assert!(original[i] <= 255);
        // assert!(original[i] >= 0);
    }
}

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // // check first arg
    // let file_name = if args.len() < 2 {
    //     debug!("Usage: {} <image>", args[0]);
    //     "frame2.jpg".to_string()
    //     // return Ok(());
    // } else {
    //     &args[1]
    // };
    let t1 = std::time::Instant::now();
    // list all the files in a directory
    let dir = std::fs::read_dir("./recording_2024-11-05_19-01-50").unwrap();
    let out_dir = "output";
    let mut all_balls = Vec::new();
    let mut file_count = 0;
    let mut ball_comp = matura::ball::BallComp::default();
    // let mut last_two_balls = (None, None);
    // let rl_compute = matura::compute_rl_coords::RLCompute::new();
    let mut files = dir.fold(vec![], |mut acc, file| {
        let file = file.unwrap();
        let file_name = file.file_name().to_str().unwrap().to_string();
        let file_path = file.path().to_str().unwrap().to_string();
        // get all numbers in the file name
        let numbers = file_name
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>();
        let number = match numbers.parse::<u32>() {
            Ok(n) => {
                acc.push((n, file_path.clone()));
            }
            Err(e) => {
                println!("Error parsing number: {}: {}", e, numbers);
            }
        };
        acc
    });
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let mut time = 0.;
    let raw_image = ImageReader::open("raw.png").unwrap().decode().unwrap();

    let raw_image = raw_image.into_rgb8();
    let raw_image = raw_image.as_raw();
    for (_, file_name) in files {
        // // let file_name = file.unwrap().path().to_str().unwrap().to_string();
        // // println!("file: {}", file_name);
        // let balls = match matura::ball::read_image(
        //     file_name.clone(),
        //     out_dir.to_string(),
        //     ball_comp.clone(),
        // ) {
        //     Ok(b) => b,
        //     Err(e) => {
        //         panic!("Error reading image: {}", e);
        //     }
        // };
        // all_balls.push(balls);
        // file_count += 1;
        // let img = ImageReader::open(file_name.clone())
        //     .unwrap()
        //     .decode()
        //     .unwrap();
        // // get pixels as [u8] with 3 channels
        // let img = img.into_rgb8();
        // let pixels = img.pixels().collect::<Vec<_>>();
        // let pixels = pixels.iter().fold(Vec::new(), |mut acc, pixel| {
        //     let p = pixel.to_rgb();
        //     acc.push(p[0]);
        //     acc.push(p[1]);
        //     acc.push(p[2]);
        //     acc
        // });
        // let t1 = std::time::Instant::now();
        // let ball = matura::ball::find_ball(&pixels, img.width(), img.height());
        // // let ball = rl_compute.transform_point((ball.0 as f32, ball.1 as f32));
        // let ball_vec2 = Vec2::new(ball.0 as f32, ball.1 as f32);

        // // append to last two balls
        // last_two_balls.0 = last_two_balls.1;
        // last_two_balls.1 = Some(ball.clone());
        // // if we have 2 balls and the velocity of ballcomp is 0, initialize it
        // // if let (Some(b1), Some(b2)) = last_two_balls {
        // //     if ball_comp.velocity.magnitude() == 0.0 {
        // //         let b1_vec2 = Vec2::new(b1.0 as f32, b1.1 as f32);
        // //         let b2_vec2 = Vec2::new(b2.0 as f32, b2.1 as f32);
        // //         ball_comp = matura::ball::BallComp::new(b1_vec2, b2_vec2, 1. / 149.);
        // //     }
        // // }
        // // compare current ball with prediction of ballcomp
        // // println!(
        // //     "Ball: {:?}, prediction: {}, diff: {}",
        // //     ball,
        // //     ball_comp.predict_position(1. / 149.),
        // //     Vec2::new(ball.0 as f32, ball.1 as f32) - ball_comp.predict_position(1. / 149.)
        // // );
        // // // update pos
        // // if let (Some(b1), Some(b2)) = last_two_balls {
        // //     let b1_vec2 = Vec2::new(b1.0 as f32, b1.1 as f32);
        // //     let b2_vec2 = Vec2::new(b2.0 as f32, b2.1 as f32);
        // //     ball_comp.check_velocity(b1_vec2, b2_vec2, 1. / 149.);
        // // }
        // ball_comp.update(ball_vec2, time);
        // // println!("Time: {:?}", t1.elapsed());
        // // println!("ball: {:?}", ball);
        // // println!("Ball at 1086: {:?}, 134", ball_comp.intersection_x(1086.));
        // // println!("Ball comp: {:?}\n\n", ball_comp);

        // read file_name
        let img = ImageReader::open(file_name.clone())
            .unwrap()
            .decode()
            .unwrap();
        let width = img.width();
        let height = img.height();
        let mut buffed_image = ImageBuffer::new(img.width(), img.height());
        for (x, y, pixel) in img.pixels() {
            buffed_image.put_pixel(x, y, pixel);
        }
        // // save to file
        // let out_file = format!(
        //     "{}/{}",
        //     out_dir,
        //     file_name.split("\\").last().unwrap().replace("png", "jpg")
        // );
        // debug!("out_file: {}", out_file);
        // buffed_image.save(out_file).unwrap();
        time += 1. / 149.;

        let img = img.to_rgb8();
        let mut img = img.as_raw().clone();
        let img = img.as_mut_slice();
        subtract_image(img, &raw_image);
        let ball = matura::ball::find_ball(img, width, height, &mut ball_comp, time);
        all_balls.push(ball);
        let img = DynamicImage::ImageRgb8(RgbImage::from_raw(width, height, img.to_vec()).unwrap());
        let out_file = file_name.replace("recording", "recording_sub");
        // check if directory exists, if not create it
        let dir_name = out_file.split("\\").collect::<Vec<&str>>()
            [0..out_file.split("\\").collect::<Vec<&str>>().len() - 1]
            .join("\\");
        if !Path::new(&dir_name).exists() {
            fs::create_dir_all(&dir_name).unwrap();
        }

        // if ball not (-1, -1, ..) aka not found we want to keep the file name, if the ball was found, the file name should be the same as the ball coordinates (without the third value)
        if ball.0 != -1 {
            let out_file = format!(
                "{}/{}",
                out_file.split("\\").collect::<Vec<&str>>()
                    [0..out_file.split("\\").collect::<Vec<&str>>().len() - 1]
                    .join("\\"),
                format!(
                    "{}_{}_{}",
                    ball.0,
                    ball.1,
                    out_file.split("\\").last().unwrap()
                )
            );
            debug!("out_file: {}", out_file);
            img.save(out_file).unwrap();
        } else {
            debug!("out_file: {}", out_file);
            img.save(out_file).unwrap();
        }
    }
    for ball in all_balls.iter() {
        // for ball in balls.iter() {
        debug!("{:?}", ball);
    }
    // }
    // println!(
    // "found {} balls in {} files",
    // all_balls.iter().map(|balls| balls.len()).sum::<usize>(),
    // file_count
    // );
    println!("Total time: {:?}", t1.elapsed());
}
