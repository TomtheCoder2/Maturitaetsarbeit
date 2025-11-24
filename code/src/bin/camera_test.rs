use matura::cam_thread::*;
use matura::image_utils::{gpu_debayer, MetalContext};

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
    set_value(&mut camera, "ExposureTime".to_string(), 2900.);
    // set_value(&mut camera, "AcquisitionFrameRate".to_string(), 300.0);
    // get_value(&mut camera, "DeviceLinkThroughputLimitMode".to_string());

    execute_command(&mut camera, "AcquisitionStop");
    set_enum_value(&mut camera, "PixelFormat", "BayerRG12p");
    execute_command(&mut camera, "AcquisitionStart");

    // Start streaming. Channel capacity is set to 3.
    let payload_rx = camera.start_streaming(3).unwrap();

    let t0 = std::time::Instant::now();
    let out_dir = std::path::Path::new("./output");
    // delete and recreate output directory
    if out_dir.exists() {
        std::fs::remove_dir_all(out_dir).unwrap();
    }
    std::fs::create_dir_all(out_dir).unwrap();
    let metal_context = MetalContext::new().expect("Metal init failed");
    let mut compute_times = vec![];
    let mut frame_count = 0usize;
    // spawn thread to wait for Enter (press Enter to stop)
    let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    {
        let stop_clone = stop.clone();
        std::thread::spawn(move || {
            let mut _buf = String::new();
            let _ = std::io::stdin().read_line(&mut _buf);
            stop_clone.store(true, std::sync::atomic::Ordering::Relaxed);
        });
    }
    println!("Streaming... press Enter to stop.");
    while !stop.load(std::sync::atomic::Ordering::Relaxed) {
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
            let image = payload.image().unwrap();
            let local_t0 = std::time::Instant::now();
            let image_rgb8 = metal_context.debayer(
                image,
                image_info.width as usize,
                image_info.height as usize,
                true,
            );
            compute_times.push(local_t0.elapsed().as_secs_f64());
            // save to file
            let filename = format!(
                "./output/camera_test_frame_{}.png",
                frame_count
            );
            matura::image_utils::save_rgb8_image(
                &filename,
                &image_rgb8,
                image_info.width as usize,
                image_info.height as usize,
            )
            .unwrap();
        } else {
            println!("No image info available");
        }
        frame_count += 1;
    }
    let elapsed = t0.elapsed().as_secs_f64();
    println!("Elapsed time for {frame_count} frames: {:.3} s", elapsed);
    println!("Average FPS: {:.2}", frame_count as f64 / elapsed);
    let avg_compute_time: f64 = compute_times.iter().sum::<f64>() / compute_times.len() as f64;
    println!(
        "Average compute time per frame: {:.3} ms",
        avg_compute_time * 1000.0
    );
}
