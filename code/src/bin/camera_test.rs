use matura::cam_thread::*;
use matura::image_utils::bayer_rg12p_to_rgb8;

fn main() {
    // construct the camera
    let mut cameras = cameleon::u3v::enumerate_cameras().unwrap();
    let mut camera = cameras.pop().unwrap();
    // Opens the camera.
    camera.open().unwrap();
    // Loads `GenApi` context. This is necessary for streaming
    camera.load_context().unwrap();
    print_all_options(&mut camera);

    // start the camera stream
    set_value(&mut camera, "ExposureTime".to_string(), 3071.);
    // set_value(&mut camera, "AcquisitionFrameRate".to_string(), 300.0);
    // get_value(&mut camera, "DeviceLinkThroughputLimitMode".to_string());

    execute_command(&mut camera, "AcquisitionStop");
    set_enum_value(&mut camera, "PixelFormat", "BayerRG12p");
    execute_command(&mut camera, "AcquisitionStart");

    // Start streaming. Channel capacity is set to 3.
    let payload_rx = camera.start_streaming(3).unwrap();

    let t0 = std::time::Instant::now();
    let n = 100;
    let out_dir = std::path::Path::new("./output");
    // delete and recreate output directory
    if out_dir.exists() {
        std::fs::remove_dir_all(out_dir).unwrap();
    }
    std::fs::create_dir_all(out_dir).unwrap();
    let mut compute_times = vec![];
    for _ in 0..n {
        // Receives next payload.
        let payload = match payload_rx.recv_blocking() {
            Ok(payload) => payload,
            Err(e) => {
                println!(
                    "[{}]Payload receive error: {e}",
                    chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
                );
                continue;
            }
        };
        if let Some(image_info) = payload.image_info() {
            // println!(
            //     "[{}]Received image: {}x{}, PixelFormat: {:?}",
            //     chrono::Local::now().format("%Y-%m-%d_%H-%M-%S"),
            //     image_info.width,
            //     image_info.height,
            //     image_info.pixel_format
            // );
            let image = payload.image().unwrap();
            let t0 = std::time::Instant::now();
            let image_rgb8 = bayer_rg12p_to_rgb8(
                image,
                image_info.width as usize,
                image_info.height as usize,
                true
            );
            compute_times.push(t0.elapsed().as_secs_f64());
            // save to file
            let filename = format!("./output/camera_test_frame_{}.png", t0.elapsed().as_millis());
            matura::image_utils::save_rgb8_image(
                &filename,
                &image_rgb8,
                image_info.width as usize,
                image_info.height as usize,
            )
            .unwrap();
        }
    }
    let elapsed = t0.elapsed().as_secs_f64();
    println!("Elapsed time for 100 frames: {:.3} s", elapsed);
    println!("Average FPS: {:.2}", n as f64 / elapsed);
    let avg_compute_time: f64 = compute_times.iter().sum::<f64>() / compute_times.len() as f64;
    println!("Average compute time per frame: {:.3} ms", avg_compute_time * 1000.0);
}
