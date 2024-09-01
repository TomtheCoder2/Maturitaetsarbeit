use std::sync::Mutex;
use std::thread;

use cameleon::Camera;
use cameleon::u3v::{ControlHandle, StreamHandle};
use eframe::Frame;
use egui::{ColorImage, TextureHandle};
use egui::Context;
use image::ImageBuffer;

pub struct App {
    file_name: String,
}

fn load_texture_from_image(ctx: &Context, image: ColorImage) -> TextureHandle {
    ctx.load_texture("my_image", image, Default::default())
}

fn create_color_image_from_rgb8(rgb8_data: &[u8], width: usize, height: usize) -> ColorImage {
    // Convert RGB8 to RGBA8 as expected by ColorImage
    let mut pixels = Vec::with_capacity(width * height * 4);

    for chunk in rgb8_data.chunks(3) {
        pixels.push(chunk[0]); // R
        pixels.push(chunk[1]); // G
        pixels.push(chunk[2]); // B
        pixels.push(255);      // A (set alpha to 255 for full opacity)
    }

    ColorImage::from_rgba_unmultiplied([width, height], &pixels)
}

static IMAGE_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui|
        {
            let buffer = IMAGE_BUFFER.lock().unwrap();
            let width = 728;
            let height = 544;
            let image = buffer.as_slice();
            let image = create_color_image_from_rgb8(image, width, height);
            let texture = load_texture_from_image(ctx, image.clone());
            ui.text_edit_singleline(&mut self.file_name);
            if ui.button("Save Image").clicked() {
                let mut file = std::fs::File::create(&self.file_name).unwrap();
                // write image to file
                let mut image_buffer = ImageBuffer::new(width as u32, height as u32);
                for x in 0..width {
                    for y in 0..height {
                        let pixel = image.pixels[y * width + x].clone();
                        image_buffer.put_pixel(x as u32, y as u32, image::Rgba::from([pixel.r(), pixel.g(), pixel.b(), pixel.a()]));
                    }
                }
                image_buffer.save(self.file_name.clone());
            }
            ui.image(&texture);
        });
        ctx.request_repaint();
    }
}

impl App {
    pub fn new() -> Self {
        App {
            file_name: "image.png".to_string(),
        }
    }
}

pub fn run_camera_test() {
    eframe::run_native(
        "Camera Test",
        Default::default(),
        Box::new(|cc| Box::new(App::new())),
    ).expect("Failed to run Camera Test");
}

fn get_value(camera: &mut Camera<ControlHandle, StreamHandle>, name: String) {
    let mut params_ctxt = camera.params_ctxt().unwrap();
    // Get `Gain` node of `GenApi`.
    // `GenApi SFNC` defines that `Gain` node should have `IFloat` interface,
    // so this conversion would be success if the camera follows that.
    // Some vendors may define `Gain` node as `IInteger`, in that case, use
    // `as_integer(&params_ctxt)` instead of `as_float(&params_ctxt).
    let exposure = params_ctxt
        .node(&*name)
        .unwrap()
        .as_enumeration(&params_ctxt).unwrap();
    println!("{:?}", exposure);

    // Get the current value of `Gain`.
    if exposure.is_readable(&mut params_ctxt).unwrap() {
        let value = exposure.entries(&mut params_ctxt);
        println!("{name}: {:?}", value);
        for value in value {
            let value_value = value.value(&mut params_ctxt).clone();
            let name = value.as_node().name(&mut params_ctxt).clone();

            println!("{}: {:?}", name, value_value);
        }
    } else {
        println!("{name} is not readable");
    }
}

fn set_value(camera: &mut Camera<ControlHandle, StreamHandle>, name: String, value: f64) {
    let mut params_ctxt = camera.params_ctxt().unwrap();
    // Get `Gain` node of `GenApi`.
    // `GenApi SFNC` defines that `Gain` node should have `IFloat` interface,
    // so this conversion would be success if the camera follows that.
    // Some vendors may define `Gain` node as `IInteger`, in that case, use
    // `as_integer(&params_ctxt)` instead of `as_float(&params_ctxt).
    let exposure = params_ctxt
        .node(&*name)
        .unwrap()
        .as_float(&params_ctxt)
        .unwrap();

    // Get the current value of `Gain`.
    if exposure.is_readable(&mut params_ctxt).unwrap() {
        let value = exposure.value(&mut params_ctxt).unwrap();
        println!("{name}: {}", value);
    }

    // Set `0.1` to `Gain`.
    if exposure.is_writable(&mut params_ctxt).unwrap() {
        exposure.set_value(&mut params_ctxt, value).unwrap();
    } else {
        println!("{name} is not writable");
    }

    // Get the current value of `Gain`.
    // The float value may be truncated to valid value by the camera.
    if exposure.is_readable(&mut params_ctxt).unwrap() {
        let value = exposure.value(&mut params_ctxt).unwrap();
        println!("New {name} {}", value);
    }
}

fn main() {
    // Enumerates all cameras connected to the host.
    let mut cameras = cameleon::u3v::enumerate_cameras().unwrap();

    if cameras.is_empty() {
        println!("no camera found");
        return;
    }

    let mut camera = cameras.pop().unwrap();

    // Opens the camera.
    camera.open().unwrap();
    // Loads `GenApi` context. This is necessary for streaming.
    camera.load_context().unwrap();

    set_value(&mut camera, "ExposureTime".to_string(), 1. * 1e6 / 30.0);
    // set_value(&mut camera, "AcquisitionFrameRate".to_string(), 300.0);
    // get_value(&mut camera, "DeviceLinkThroughputLimitMode".to_string());


    // Start streaming. Channel capacity is set to 3.
    let payload_rx = camera.start_streaming(3).unwrap();

    thread::spawn(move || {
        let mut t0 = std::time::Instant::now();
        let mut frame_counter = 0;
        loop {
            frame_counter += 1;
            if frame_counter % 1000 == 0 {
                // show rolling fps average
                let elapsed = t0.elapsed();
                let fps = frame_counter as f64 / elapsed.as_secs_f64();
                println!("fps: {:.2}", fps);
            }
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
                let width = image_info.width;
                let height = image_info.height;

                let image = payload.image();
                if let Some(image) = image {
                    // save to IMAGE_BUFFER
                    let mut buffer = IMAGE_BUFFER.lock().unwrap();
                    buffer.clear();
                    buffer.extend_from_slice(image);
                }
            }
        }
    });

    run_camera_test();
}