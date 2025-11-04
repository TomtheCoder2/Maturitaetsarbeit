extern crate core;
use image::{DynamicImage, ImageBuffer};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use matura::ball;
use matura::ball::{standard_selection, BallComp};
use matura::cam_thread::{load_raw, HEIGHT, MIN_X, MIN_Y, NEW_HEIGHT, NEW_WIDTH, WIDTH};
use matura::live_feed::subtract_image;
use matura::plot::PlotApp;

fn main() {
    let test_folder = "./recording_4";
    // lets read all images from the folder
    let mut files = std::fs::read_dir(test_folder)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("jpeg") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    // sort the files by number in the filename
    files.sort_by_key(|path| {
        let filename = path.file_stem().and_then(|s| s.to_str()).unwrap();
        let number_str = filename.split('_').last().unwrap();
        number_str.parse::<u32>().unwrap()
    });
    println!("files: {:?}", files);
    let precompute = matura::gen_table(WIDTH, HEIGHT, NEW_WIDTH, NEW_HEIGHT, MIN_X, MIN_Y);
    let raw = load_raw();
    // we want to convert it to rgb from rgba, so we delete every 4th element
    let raw_image = raw
        .as_raw()
        .to_vec()
        .chunks(4)
        .map(|x| x[0..3].to_vec())
        .flatten()
        .collect::<Vec<u8>>();
    let mut raw_image = raw_image.as_slice();
    let mut undistorted_image = vec![0u8; (NEW_WIDTH * NEW_HEIGHT * 3) as usize];
    // let mut original_undistorted_image = DynamicImage::new(NEW_WIDTH, NEW_HEIGHT, ColorType::Rgb8);
    let mut ball_comp = BallComp::new();
    let mut selection_fn: Box<dyn Fn(u8, u8, u8) -> bool> = Box::new(standard_selection);
    let min_radius = 10.;
    let max_radius = 20.;
    let player_x_coord = 30;
    let start = std::time::Instant::now();
    let pb = ProgressBar::new(files.len() as u64).with_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    let mut overlay_images = vec![];
    let mut intersections = vec![];
    let mut actual_intersection = 0.;
    let mut ball_positions = vec![];
    let mut latest_prediction = 0;
    let mut elapsed_find_total = 0.;
    let file_number = files.len();
    for file in files {
        pb.inc(1);
        // read file as image
        let image = image::open(&file).unwrap().to_rgb8();
        let (width, height) = image.dimensions();
        let image = image.to_vec();

        matura::undistort_image_table(
            &*image,
            &mut undistorted_image,
            &precompute,
            NEW_WIDTH,
            NEW_HEIGHT,
        );

        let mut original_undistorted_image = DynamicImage::ImageRgb8(
            ImageBuffer::from_raw(NEW_WIDTH, NEW_HEIGHT, undistorted_image.clone()).unwrap(),
        );

        subtract_image(&mut undistorted_image, raw_image);

        let elapsed = start.elapsed();
        let t0 = std::time::Instant::now();
        let ball_t = matura::ball::find_ball(
            undistorted_image.as_slice(),
            NEW_WIDTH as u32,
            NEW_HEIGHT as u32,
            &mut ball_comp,
            elapsed.as_secs_f32(),
            &selection_fn,
            min_radius,
            max_radius,
        );
        let elapsed_find = t0.elapsed();
        elapsed_find_total += elapsed_find.as_secs_f32();
        let time = start.elapsed().as_secs_f32();
        let mut undistorted_dyn_image = DynamicImage::ImageRgb8(
            ImageBuffer::from_raw(NEW_WIDTH, NEW_HEIGHT, undistorted_image.clone()).unwrap(),
        );
        // let mut original_undistorted_image = undistorted_dyn_image.clone();
        let _ = ball::read_image_vis(
            &mut undistorted_dyn_image,
            &mut original_undistorted_image,
            &mut ball_comp,
            time,
            &selection_fn,
            min_radius,
            max_radius,
            true,
        );
        overlay_images.push(original_undistorted_image.clone());
        #[derive(Copy, Clone)]
        struct Ball {
            x: i32,
            y: i32,
            radius: f32,
        }
        let mut ball = Ball {
            x: ball_t.0,
            y: ball_t.1,
            radius: ball_t.2,
        };
        if ball_t != (-1, -1, -1.) {
            // println!(
            //     "Found ball at ({}, {}), radius: {:.2} in file {:?}",
            //     ball.x, ball.y, ball.radius, file
            // );
            ball_positions.push(ball);
            // lets see where the ball will hit the player x coordinate
            let intersection = ball_comp.intersection_x(player_x_coord as f32);
            if let Some(intersection) = intersection {
                if ball_comp.velocity.x < 0.0 && ball_comp.velocity.magnitude() > 20. {
                    println!("intersection: {:?}", intersection);
                    intersections.push(intersection);
                }
            }
        } else {
            // println!("No ball found in file {:?}", file);
        }

        if ball.x > player_x_coord {
            latest_prediction = ball.y;
        }
    }
    pb.finish_with_message("Processed all images");
    ball_positions.sort_by(|b1, b2| {
        b1.x.abs_diff(player_x_coord)
            .cmp(&b2.x.abs_diff(player_x_coord))
    });
    if let Some(closest_ball) = ball_positions.first() {
        actual_intersection = closest_ball.y as f32;
        println!(
            "Actual intersection at y = {:.2}, x = {:.2}",
            actual_intersection, closest_ball.x
        );
    } else {
        println!("No ball positions recorded.");
    }
    println!("intersections: {:?}", intersections);
    let mut error = 0;
    for intersection in &intersections {
        error += (intersection.0.y - actual_intersection).abs() as u32;
    }
    let avg_error = error as f32 / intersections.len() as f32;
    // WIDTH in irl is 700mm
    let avg_error_mm = avg_error * (700.0 / NEW_WIDTH as f32);
    println!(
        "Average intersection error: {:.2} pixels, {:.2} mm",
        avg_error, avg_error_mm
    );

    let error_latest = (latest_prediction as f32 - actual_intersection).abs();
    let error_latest_mm = error_latest * (700.0 / NEW_WIDTH as f32);
    println!(
        "Latest prediction error: {:.2} pixels, {:.2} mm",
        error_latest, error_latest_mm
    );

    let plot_data = intersections
        .iter()
        .map(|p| {
            [
                p.1 as f64 * (700.0 / NEW_WIDTH as f64),
                (p.0.y as f64 - actual_intersection as f64).abs(),
            ]
        })
        .collect::<Vec<[f64; 2]>>();

    println!("elapsed_find_total: {:.2} seconds", elapsed_find_total);
    println!(
        "Average time per frame for ball finding: {:.3} ms",
        (elapsed_find_total / file_number as f32) * 1000.0
    );

    let dir_name = format!("ball_prediction_test_output");
    std::fs::create_dir_all(dir_name.clone()).unwrap();
    // save images to file
    println!("Saving images to folder: {}", dir_name);
    let total = overlay_images.len();
    if total == 0 {
        println!("No images to save");
    } else {
        // let pb show the eta
        let pb = ProgressBar::new(total as u64).with_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("##-"),
        );
        overlay_images
            .par_iter()
            .enumerate()
            .for_each(|(i, image)| {
                let path = format!("{}/image_{}.jpeg", dir_name, i);
                image
                    .save_with_format(path, image::ImageFormat::Jpeg)
                    .unwrap();
                pb.inc(1);
            });
        pb.finish_with_message("Saved all images");
    }

    plot_main(plot_data);
}

fn plot_main(graph: Vec<[f64; 2]>) {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
        ..Default::default()
    };
    // let graph: Vec<[f64; 2]> = vec![[0.0, 1.0], [2.0, 3.0], [3.0, 2.0]];
    let graph2: Vec<[f64; 2]> = vec![];
    let graph3: Vec<[f64; 2]> = vec![];

    eframe::run_native(
        "My egui App with a plot",
        options,
        Box::new(|_cc| {
            Ok(Box::new(PlotApp {
                insert_order: false,
                graph,
                graph2,
                graph3,
            }))
        }),
    )
    .unwrap()
}
