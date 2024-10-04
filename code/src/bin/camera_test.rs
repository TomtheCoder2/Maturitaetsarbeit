extern crate core;

use cameleon::payload::PayloadReceiver;
use cameleon::u3v;
use cameleon::u3v::{ControlHandle, StreamHandle};
use eframe::Frame;
use egui::Context;
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

    let n = 10;
    for _ in 0..n {
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
            }
        }

        // Send back payload to streaming loop to reuse the buffer. This is optional.
        payload_rx.send_back(payload);
    }

    let t2 = std::time::Instant::now();
    println!("Time taken: {:?}", t2 - t1);
    println!("FPS: {}", n as f64 / (t2 - t1).as_secs_f64());

    // if the foler images doesnt exist, create it
    std::fs::create_dir_all("images").unwrap();

    // save images to file
    for (i, image) in images.iter().enumerate() {
        save_buffer_with_format(
            format!("images/image_{}.jpg", i),
            &image,
            width as u32,
            height as u32,
            image::ColorType::Rgb8,
            image::ImageFormat::Png,
        )
        .unwrap();
    }

    // Closes the camera.
    camera.close().unwrap();
}
