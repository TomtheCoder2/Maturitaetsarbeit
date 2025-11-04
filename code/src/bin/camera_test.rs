extern crate core;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use cameleon::u3v;
use image::save_buffer_with_format;

fn main() {
    // Enumerates all cameras connected to the host.
    let mut cameras = u3v::enumerate_cameras().unwrap();

    if cameras.is_empty() {
        println!("no camera found");
        return;
    }

    let mut camera = cameras.pop().unwrap();

    // Opens the camera.
    camera.open().unwrap();
    // Loads `GenApi` context. This is necessary for streaming.
    camera.load_context().unwrap();

    // Start streaming. Channel capacity is set to 3.
    let payload_rx = camera.start_streaming(3).unwrap();

    let mut images = vec![];
    let mut width = 0;
    let mut height = 0;
    let t1 = std::time::Instant::now();

    let mut n = 0;
    let mut last_time = std::time::Instant::now();
    let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let stop_clone = stop.clone();
    std::thread::spawn(move || {
        let mut input = String::new();
        println!("Press Enter to stop streaming...");
        let _ = std::io::stdin().read_line(&mut input);
        stop_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    });

    while !stop.load(std::sync::atomic::Ordering::SeqCst) {
        let payload = match payload_rx.recv_blocking() {
            Ok(payload) => payload,
            Err(e) => {
                println!("payload receive error: {e}");
                continue;
            }
        };
        // println!(
        //     "payload received! block_id: {:?}, timestamp: {:?}",
        //     payload.id(),
        //     payload.timestamp()
        // );
        if let Some(image_info) = payload.image_info() {
            // println!("{:?}\n", image_info);
            width = image_info.width;
            height = image_info.height;

            let image = payload.image();
            if let Some(image) = image {
                // save image to images
                images.push(image.to_vec());
                n += 1;
            }
        }

        // let delta = last_time.elapsed();
        // println!("Frame time: {:?}, fps: {}", delta, 1.0 / delta.as_secs_f64());
        last_time = std::time::Instant::now();

        // Send back payload to streaming loop to reuse the buffer. This is optional.
        payload_rx.send_back(payload);
    }

    let t2 = std::time::Instant::now();
    println!("Time taken: {:?}", t2 - t1);
    println!("FPS: {}", n as f64 / (t2 - t1).as_secs_f64());
    println!("Total frames captured: {}", n);

    // if the foler images doesnt exist, create it
    // dir name with current timestamp (in this format: recording_DDMMYYYY_HHMMSS)
    let dir_name = format!("recording_{}", chrono::Local::now().format("%d%m%Y_%H%M%S"));
    std::fs::create_dir_all(dir_name.clone()).unwrap();

    // Closes the camera.
    camera.close().unwrap();

    // save images to file
    println!("Saving images to folder: {}", dir_name);
    let total = images.len();
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
        images.par_iter().enumerate().for_each(|(i, image)| {
            let path = format!("{}/image_{}.jpeg", dir_name, i);
            save_buffer_with_format(
                path,
                &image,
                width as u32,
                height as u32,
                image::ColorType::Rgb8,
                image::ImageFormat::Jpeg,
            )
            .unwrap();
            pb.inc(1);
        });
        pb.finish_with_message("Saved all images");
    }
}
