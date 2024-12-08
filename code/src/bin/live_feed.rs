use matura::compute_rl_coords::RLCompute;
use std::f32::consts::PI;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::Instant;

use cameleon::u3v::{ControlHandle, StreamHandle};
use cameleon::Camera;
use eframe::Frame;
use egui::Context;
use egui::{ColorImage, TextureHandle};
use matura::arduino_com::ArduinoCom;
use matura::ball::BallComp;
use matura::increment_last_number_in_filename;
use std::sync::atomic::Ordering;

use crate::Command::*;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Command {
    Exposure(f64),
    Start,
    Stop,
    Pause,
    Reset,
    ReloadRaw,
    MoveCenter,
    PlayerCalibration(i32),
    FinishPlayerCalibration(Vec<i32>),
    Shoot,
    ResetDC,
}

#[derive(Debug)]
pub enum Mode {
    Normal,
    PlayerCalibration,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    file_name: String,
    #[serde(skip)]
    pub sender: Sender<Command>,
    exposure: f64,
    auto_exposure: bool,
    calibration_mode: bool,
    #[serde(skip)]
    last_cal_image: Instant,
    calibration_interval: f64,
    paused: bool,
    #[serde(skip)]
    recorded_images: Vec<ColorImage>,
    #[serde(skip)]
    recording: bool,
    #[serde(skip)]
    raw_image: ColorImage,
    show_original: bool,
    overlay_raw_img: bool,
    #[serde(skip)]
    ball_comp: BallComp,
    #[serde(skip)]
    start_time: Instant,
    #[serde(skip)]
    compute_rl_coords: matura::compute_rl_coords::RLCompute,
    #[serde(skip)]
    last_command: Instant,
    speed: i32,
    motor_pos: i32,
    #[serde(skip)]
    last_frame: Instant,
    overlay_ball: bool,
    show_player_predicition: bool,
    pause_player: bool,
    pause_shooting: bool,
    #[serde(skip)]
    click_position: Option<egui::emath::Vec2>,
    #[serde(skip)]
    mode: Mode,
    #[serde(skip)]
    player_calibration_pos: i32,
    final_player_calibration_positions: Vec<i32>,
    #[serde(skip)]
    player_calibration_message: String,
    followball: bool
}

const POS: [i32; 7] = [50, 100, 150, 200, 250, 300, 350];

impl Default for App {
    fn default() -> Self {
        let (sender, _) = mpsc::channel();
        App {
            file_name: "my_image.png".to_owned(),
            sender,
            exposure: 0.0,
            auto_exposure: true,
            calibration_mode: false,
            last_cal_image: Instant::now(),
            calibration_interval: 0.5,
            paused: false,
            recorded_images: Vec::new(),
            recording: false,
            // read the image from ./raw.png
            raw_image: load_raw(),
            show_original: false,
            overlay_raw_img: false,
            ball_comp: BallComp::default(),
            start_time: Instant::now(),
            compute_rl_coords: matura::compute_rl_coords::RLCompute::new(),
            last_command: Instant::now(),
            speed: 100,
            motor_pos: 0,
            last_frame: Instant::now(),
            overlay_ball: true,
            show_player_predicition: true,
            pause_player: false,
            pause_shooting: false,
            click_position: None,
            mode: Mode::Normal,
            player_calibration_pos: 0,
            final_player_calibration_positions: Vec::new(),
            player_calibration_message: "".to_string(),
            followball: false
        }
    }
}

fn load_raw() -> ColorImage {
    let image = image::open("./raw.png").expect("Failed to open image");
    let rgb8_data = image.to_rgb8().into_raw();
    let width = image.width() as usize;
    let height = image.height() as usize;
    println!("Loaded raw image: {}x{}", width, height);
    create_color_image_from_rgb8(&rgb8_data, width, height)
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
        pixels.push(255); // A (set alpha to 255 for full opacity)
    }

    ColorImage::from_rgba_unmultiplied([width, height], &pixels)
}

static IMAGE_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static IMAGE_BUFFER_UNDISTORTED: Mutex<(u32, u32, Vec<u8>)> = Mutex::new((0, 0, Vec::new()));
static FPS: Mutex<f64> = Mutex::new(0.0);

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // println!("Loading image...");
            let buffer = match IMAGE_BUFFER.lock() {
                Ok(buffer) => buffer.clone(),
                Err(_) => {
                    ui.label("No image data available");
                    ui.label(format!("FPS: {:>5.0}", 1. / (*FPS.lock().unwrap())));
                    return;
                }
            };
            let buffer_undistorted = match IMAGE_BUFFER_UNDISTORTED.try_lock() {
                Ok(buffer) => buffer.clone(),
                Err(_) => {
                    ui.label("No image data available");
                    return;
                }
            };
            let actual_player_pos = ACTUAL_PLAYER_POSITION.lock().unwrap().clone();
            let comp_fps = 1. / self.last_frame.elapsed().as_secs_f64();
            self.last_frame = Instant::now();
            let player_detection_fps = PLAYER_DETECTION_FPS.load(Ordering::Relaxed);
            ui.label(format!("Comp FPS: {:>5.0}, actual player pos: {}:{}, player detection fps: {}", comp_fps, actual_player_pos.0, actual_player_pos.1, player_detection_fps));
            // println!(
            // "Image loaded: {}x{}",
            // buffer_undistorted.0, buffer_undistorted.1
            // );
            if buffer.is_empty() || buffer_undistorted.2.is_empty() {
                ui.label("No image data available");
                return;
            }
            let width = 728;
            let height = 544;
            let image = buffer_undistorted.2.as_slice();

            let mut image = DynamicImage::ImageRgb8(
                ImageBuffer::from_raw(buffer_undistorted.0, buffer_undistorted.1, image.to_vec())
                    .unwrap(),
            );

            let new_height = image.height();
            let new_width =
                (image.width() as f32 / image.height() as f32 * new_height as f32) as u32;
            // let mut image =
            // image.resize(new_width, new_height, image::imageops::FilterType::Nearest);
            let mut original_undistorted_image = image.clone();
            let unmodified_original_undistorted_image = image.clone();
            let original_image = DynamicImage::ImageRgb8(
                ImageBuffer::from_raw(width, height, buffer.to_vec()).unwrap(),
            );
            let mut subtracted_image = image.clone();
            // subract self.raw_image from image
            // if self.overlay_raw_img {
            let raw_image = &self.raw_image;
            for x in 0..raw_image.width() {
                for y in 0..raw_image.height() {
                    let raw_pixel = raw_image.pixels[y * raw_image.width() + x];
                    if x >= subtracted_image.width() as usize || y >= subtracted_image.height() as usize {
                        // println!("Pixel out of bounds: {}x{}", x, y);
                        continue;
                    }
                    let image_pixel = subtracted_image.get_pixel(x as u32, y as u32);
                    let r = image_pixel[0] as i32 - raw_pixel[0] as i32;
                    let g = image_pixel[1] as i32 - raw_pixel[1] as i32;
                    let b = image_pixel[2] as i32 - raw_pixel[2] as i32;
                    subtracted_image.put_pixel(
                        x as u32,
                        y as u32,
                        image::Rgba([
                            (r.max(0) as u8).min(255),
                            (g.max(0) as u8).min(255),
                            (b.max(0) as u8).min(255),
                            255,
                        ]),
                    );
                }
            }
            // }
            // dbg!();
            // let original_image =
            // original_image.resize(width, height, image::imageops::FilterType::Nearest);
            if self.overlay_raw_img {
                original_undistorted_image = subtracted_image.clone();
                image = subtracted_image.clone();
            }

            let time = Instant::now().duration_since(self.start_time).as_secs_f32();
            let ball = matura::ball::read_image_vis(&mut subtracted_image, &mut original_undistorted_image, &mut self.ball_comp, time);
            // println!("ball: {:?}", ball);
            // if !self.overlay_raw_img {
            if self.overlay_ball {
                image = original_undistorted_image;
            }

            let height = image.height();
            let width = image.width();
            if self.paused {
                // draw 2 black bars on top (as pause symbol)
                for x in 0..width {
                    for y in 0..height {
                        if y < height / 10 || y > height / 10 * 9 {
                            image.put_pixel(x, y, image::Rgba([255, 0, 0, 0]));
                        }
                    }
                    // make grayscale
                    // let gray = (pixel.2[0] as f32 * 0.3
                    //     + pixel.2[1] as f32 * 0.59
                    //     + pixel.2[2] as f32 * 0.11) as u8;
                    // pixel.2 = image::Rgba([255, 255, 255, gray]);
                }
            }
            // }
            // if let Some(pos) = self.arduino_com.try_receive_command() {
            // if let com::commands::Command::Pos(p) = pos {
            // self.motor_pos = p;
            // }
            // }
            // ui.label(format!("Motor pos: {}", self.motor_pos));
            let mut player_final_pos = 0;
            // x = 102 is the player
            if let Some(y_intercept) = self.ball_comp.intersection_x(44.) {
                let y_intercept = y_intercept.0;
                let rl_y_intercept = self.compute_rl_coords.transform_point((y_intercept.x, y_intercept.y));
                // y=0 for the player is at 450mm rl coords
                let player_y = 450. - rl_y_intercept.1;
                // let ball_irl = self.compute_rl_coords.transform_point((ball.0 as f32, ball.1 as f32));
                // let player_y = 450. - ball_irl.1;
                // ui.horizontal (|ui|{ui.label(format!("y intercept: {:.2}, {:.2}", y_inercept.x, y_inercept.y));
                // ui.label(format!("Player pos: y: {:.2}", player_y));});
                if player_y > 5. && player_y < 140. && self.last_command.elapsed().as_secs_f32() > 0.001 {
                    // gear: diameter = 83mm, 200 steps per revolution, 1
                    // c
                    // 8° per step
                    // let rot = player_y / (PI * 64.) * 200.;
                    let x = 1058.82 - 2.35 * rl_y_intercept.1;
                    // println!("pos: {player_y}");
                    // if rot < 30. {
                    // self.arduino_com.send_command(com::commands::Command::Reset(40));
                    // }
                    // self.arduino_com.send_stepper_motor_speed(self.speed);
                    // self.arduino_com.send_stepper_motor_pos(rot as i32);
                    // self.arduino_com.send_string(&format!("{}", x as i32));
                    // self.last_command = Instant::now();
                    player_final_pos = x as i32;
                }
            }
            // motor, y
            // 0   -> 450 mm
            // 400 -> 280 mm
            //
            // 450 - y = 450 - 280 / 400 * x
            // 450 - y = 170 / 400 * x
            // 450 - y = 0.425 * x
            // x = (450 - y) / 0.425
            // x = 1058.82 - 2.35 * y
            // y = 450 - 0.425 * x

            let ball_irl = self.compute_rl_coords.transform_point((ball.0 as f32, ball.1 as f32));
            let player_y = 450. - ball_irl.1;
            // ui.horizontal (|ui|{ui.label(format!("y intercept: {:.2}, {:.2}", y_inercept.x, y_inercept.y));
            // ui.label(format!("Player pos: y: {:.2}", player_y));});
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Ball: x: {:.2}, y: {:.2}, radius: {:.2}, velocity: x: {:.2}, y: {:.2}, magnitude: {:.2}",
                    ball.0, ball.1, ball.2, self.ball_comp.velocity.x, self.ball_comp.velocity.y, self.ball_comp.velocity.magnitude()
                ));
                ui.label(format!("Ball irl: x: {:.2}, y: {:.2}", ball_irl.0, ball_irl.1));
                ui.label(format!("Player pos: y: {:.2}", player_y));
            });
            if player_y > 5. && player_y < 140. && self.last_command.elapsed().as_secs_f32() > 0.01 {
                // gear: diameter = 83mm, 200 steps per revolution, 1
                // c
                // 8° per step
                let rot = player_y / (PI * 64.) * 200.;
                let x = 1058.82 - 2.35 * ball_irl.1;
                player_final_pos = x as i32;
                // println!("pos: {player_y}");
                // if rot < 30. {
                // self.arduino_com.send_command(com::commands::Command::Reset(40));
                // }
                // self.arduino_com.send_stepper_motor_speed(self.speed);
                // self.arduino_com.send_stepper_motor_pos(rot as i32);
                // self.arduino_com.send_string(&format!("{}", player_final_pos));
                // self.last_command = Instant::now();
            }

            if self.show_player_predicition {
                matura::ball::draw_circle(&mut image, 100, actual_player_pos.1, 5., [255, 0, 0, 255]);
                // draw the line y = actual_player_pos.1
                for x in 0..image.width() {
                    image.put_pixel(x, actual_player_pos.1 as u32, image::Rgba([255, 0, 0, 255]));
                }
            }

            let ci_image = ColorImage::from_rgb(
                [image.width() as usize, image.height() as usize],
                image.as_bytes(),
            );
            let undistorted_texture = load_texture_from_image(ctx, ci_image.clone());
            let original_texture = load_texture_from_image(
                ctx,
                ColorImage::from_rgb(
                    [
                        original_image.width() as usize,
                        original_image.height() as usize,
                    ],
                    original_image.as_bytes(),
                ),
            );
            if self.recording {
                self.recorded_images.push(ci_image.clone());
            }

            if self.auto_exposure {
                // caluclate average brightness
                let mut brightness = 0.0;
                for i in 0..ci_image.pixels.len() {
                    let pixel = ci_image.pixels[i];
                    brightness += pixel.r() as f64 + pixel.g() as f64 + pixel.b() as f64;
                }
                brightness /= ci_image.pixels.len() as f64;
                let target_brightness = 0.5 * 255.0 * 3.0;
                println!(
                    "brightness: {}, target_brightness: {}",
                    brightness, target_brightness
                );
                let diff = target_brightness - brightness;
                let adjujustment_factor = 0.05;
                self.exposure += diff * adjujustment_factor;
                self.sender.send(Command::Exposure(self.exposure)).unwrap();
            }
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.file_name);
                fn save_img(image: DynamicImage, file_name: String) {
                    let width = image.width() as usize;
                    let height = image.height() as usize;
                    // write image to file
                    let mut image_buffer = ImageBuffer::new(width as u32, height as u32);
                    for x in 0..width as usize {
                        for y in 0..height as usize {
                            let pixel = image.get_pixel(x as u32, y as u32);
                            image_buffer.put_pixel(
                                x as u32,
                                y as u32,
                                image::Rgba::from([pixel[0], pixel[1], pixel[2], 255]),
                            );
                        }
                    }
                    image_buffer
                        .save(if !file_name.clone().ends_with("png") {
                            file_name.clone() + ".png"
                        } else {
                            file_name.clone()
                        })
                        .expect("Could not save image");
                }
                if ui.button("Save Image").clicked()
                    || ui.input(|ui| ui.key_pressed(egui::Key::Enter))
                    || (self.calibration_mode
                    && Instant::now()
                    .duration_since(self.last_cal_image)
                    .as_secs_f64()
                    > self.calibration_interval)
                {
                    save_img(image.clone(), self.file_name.clone());
                    save_img(original_image, self.file_name.clone() + "_raw");
                    if self.calibration_mode {
                        self.file_name = increment_last_number_in_filename(&self.file_name)
                            .expect("Could not increment last number in filename");
                        self.last_cal_image = Instant::now();
                    }
                }
                if ui.button("Save Raw overlay").clicked() {
                    save_img(unmodified_original_undistorted_image.clone(), "./raw.png".to_string());
                    self.raw_image = load_raw();
                    self.sender.send(ReloadRaw);
                }
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.speed, 0..=1000));
                if ui.button("Reset").clicked() {
                    // self.arduino_com.send_command(com::commands::Command::Reset(0));
                    // self.arduino_com.send_string("R");
                    self.sender.send(Reset).unwrap();
                }
                if ui.button("Reset DC").clicked() {
                    self.sender.send(Command::ResetDC).unwrap();
                }
                if ui.button("Move to center").clicked() {
                    self.sender.send(Command::MoveCenter).unwrap();
                }
                if ui.button("Shoot").clicked() {
                    self.sender.send(Command::Shoot).unwrap();
                }
                if ui.checkbox(&mut self.pause_player, "Pause player movement").clicked() {
                    PAUSEPLAYER.store(self.pause_player, Ordering::Relaxed);
                }
                if ui.checkbox(&mut self.pause_shooting, "Pause shooting").clicked() {
                    PAUSEPLAYER.store(self.pause_shooting, Ordering::Relaxed);
                }
            });
            ui.horizontal(|ui| {
                ui.label("Exposure: ");
                if ui
                    .add(egui::DragValue::new(&mut self.exposure).speed(0.01))
                    .clicked()
                    || ui.button("Set Exposure").clicked()
                {
                    self.sender.send(Command::Exposure(self.exposure)).unwrap();
                }
                ui.checkbox(&mut self.show_original, "Show original Image");
                ui.checkbox(&mut self.auto_exposure, "Auto Exposure");
                ui.checkbox(&mut self.calibration_mode, "Calibration Mode");
                ui.checkbox(&mut self.overlay_raw_img, "Overlay Raw Image");
                ui.checkbox(&mut self.overlay_ball, "Overlay Ball");
                ui.checkbox(&mut self.show_player_predicition, "Show Player Prediction");
                if ui.checkbox(&mut self.followball, "Follow Ball").clicked() {
                    FOLLOWBALL.store(self.followball, Ordering::Relaxed);
                }
                if matches!(self.mode, Mode::PlayerCalibration) {
                    ui.label("Click on Player!");
                    if self.player_calibration_message.len() > 0 {
                        ui.label(&self.player_calibration_message);
                    }
                    if ui.button("Next").clicked() || ui.input(|ui| ui.key_pressed(egui::Key::S)) {
                        if self.click_position.is_none() {
                            self.player_calibration_message = "Click on the player!".to_string();
                        } else {
                            let pos = self.click_position.unwrap();
                            let posy = pos.y as i32;
                            self.player_calibration_message = format!("pos: {}, {}/{}", posy, self.player_calibration_pos, POS.len());
                            // if self.player_calibration_pos >= 0 {
                            self.final_player_calibration_positions.push(posy);
                            // }
                            println!("Player pos: y:{}", posy);
                            self.player_calibration_pos += 1;
                            self.sender.send(Command::PlayerCalibration(POS[self.player_calibration_pos as usize]));
                            if self.player_calibration_pos >= POS.len() as i32 - 1 {
                                self.mode = Mode::Normal;
                                println!("Player calibration finished");
                                println!("pos: {:?}", self.final_player_calibration_positions);
                                println!("Player calibration positions: {}", self.final_player_calibration_positions.iter().enumerate().map(|(i, x)| format!("\t{}: {}: {}\n", i, POS[i], x)).collect::<Vec<String>>().join(""));
                                self.sender.send(Command::FinishPlayerCalibration(self.final_player_calibration_positions.clone()));
                                PAUSEPLAYER.store(false, Ordering::Relaxed);
                                self.pause_player = false;
                            } else {}
                        }
                    }
                } else {
                    if ui.button("Player Calibration").clicked() {
                        self.mode = Mode::PlayerCalibration;
                        self.player_calibration_pos = -1;
                        PAUSEPLAYER.store(true, Ordering::Relaxed);
                        self.pause_player = true;
                        self.sender.send(Command::PlayerCalibration(-1));
                        self.final_player_calibration_positions.clear();
                    }
                }
                if self.calibration_mode {
                    ui.label("Calibration Image Interval: ");
                    ui.add(egui::DragValue::new(&mut self.calibration_interval).speed(0.1));
                }
                if ui
                    .button(format!("{}", if self.paused { "Play" } else { "Pause" }))
                    .clicked()
                    || ui.input(|ui| ui.key_pressed(egui::Key::Space))
                {
                    self.paused = !self.paused;
                    self.sender
                        .send(if self.paused {
                            Command::Pause
                        } else {
                            Command::Start
                        })
                        .unwrap();
                }
                if ui
                    .button(format!(
                        "{}",
                        if self.recording { "Stop" } else { "Record" }
                    ))
                    .clicked()
                    || ui.input(|ui| ui.key_pressed(egui::Key::R))
                {
                    self.recording = !self.recording;
                    // turn of calibration mod
                    if self.recording {
                        self.calibration_mode = false;
                    }
                    // if recording is stopped save images to file
                    if !self.recording {
                        let mut prefix = format!(
                            "./recording_{}/",
                            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
                        );
                        std::fs::create_dir_all(&prefix).unwrap();
                        for (i, image) in self.recorded_images.iter().enumerate() {
                            let mut file_name = prefix.clone();
                            file_name.push_str(&format!("{:04}.png", i));
                            let mut file = std::fs::File::create(&file_name).unwrap();
                            // write image to file
                            let mut image_buffer = ImageBuffer::new(width as u32, height as u32);
                            for x in 0..width as usize {
                                for y in 0..height as usize {
                                    let pixel = image.pixels[y * width as usize + x].clone();
                                    image_buffer.put_pixel(
                                        x as u32,
                                        y as u32,
                                        image::Rgba::from([
                                            pixel.r(),
                                            pixel.g(),
                                            pixel.b(),
                                            pixel.a(),
                                        ]),
                                    );
                                }
                            }
                            image_buffer.save(file_name).expect("Could not save image");
                        }
                        println!(
                            "Recording saved to {}, saved {} images",
                            prefix,
                            self.recorded_images.len()
                        );
                        self.recorded_images.clear();
                    }
                }
            });
            // ui.image(&texture);
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    if self.show_original {
                        ui.add(
                            egui::Image::new(&original_texture), // .max_width(200.0).rounding(10.0)
                        );
                    }


                    let response = ui.image(&undistorted_texture).interact(egui::Sense::click());

                    if response.clicked() {
                        if let Some(pos) = response.hover_pos() {
                            // Get the click position relative to the image
                            let local_pos = pos - response.rect.min;
                            self.click_position = Some(local_pos);

                            // Log the pixel coordinates
                            println!("Clicked at: ({}, {})", local_pos.x as u32, local_pos.y as u32);
                        }
                    }

                    // Draw the red dot where the user clicked
                    if let Some(pos) = self.click_position {
                        let screen_pos = response.rect.min + pos;
                        ui.painter()
                            .circle_filled(screen_pos, 5.0, egui::Color32::RED);
                    }
                });
            });
            ui.label(format!("FPS: {:>5.0}", 1. / (*FPS.lock().unwrap())));
        });
        ctx.request_repaint();
    }

    /// Called by the frame work to save state before shutdown.
    /// On Windows its saved here: C:\Users\UserName\AppData\Roaming\Phoenix\data\app.ron
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // self.version = VERSION.to_string();
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl App {
    pub fn new(tx: Sender<Command>, cc: &eframe::CreationContext<'_>) -> Self {
        let mut app;
        if let Some(storage) = cc.storage {
            app = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        } else {
            app = App::default();
        };
        app.sender = tx;
        // send current exposure to camera
        app.sender.send(Command::Exposure(app.exposure)).unwrap();
        PAUSEPLAYER.store(app.pause_player, Ordering::Relaxed);
        FOLLOWBALL.store(app.followball, Ordering::Relaxed);
        PAUSESHOOTING.store(app.pause_shooting, Ordering::Relaxed);
        if app.paused {
            app.sender.send(Command::Pause).unwrap();
        } else {
            app.sender.send(Command::Start).unwrap();
        }
        app
    }
}

pub fn run_camera_test(tx: Sender<Command>) {
    println!("Starting Camera Test GUI");
    eframe::run_native(
        "Camera Test",
        Default::default(),
        Box::new(|cc| Box::new(App::new(tx, cc))),
    )
        .expect("Failed to run Camera Test");
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
        .as_enumeration(&params_ctxt)
        .unwrap();
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

    // let mut params_ctxt = camera.params_ctxt().unwrap();
    // params_ctxt.node_store().visit_nodes(|f|
    //         // IntSwissKnife(IntSwissKnifeNode { attr_base: NodeAttributeBase { id: NodeId(2247), name_space: Custom, merge_priority: Mid, expose_static: None }, elem_base: NodeElementBase { tooltip: None, description: None, display_name: None, visibility: Beginner, docu_url: None, is_deprecated: false, event_id: None, p_is_implemented: None, p_is_available: None, p_is_locked: None, p_block_polling: None, imposed_access_mode: RW, p_errors: [], p_alias: None, p_cast_alias: None, p_invalidators: [] }, streamable: false, p_variables: [NamedValue { name: "CORRECTIONSELECTORINDEX", value: NodeId(1365) }], constants: [], expressions: [], formula: Formula { expr: BinOp { kind: Mul, lhs: BinOp { kind: Add, lhs: Integer(386), rhs: Ident("CORRECTIONSELECTORINDEX") }, rhs: Integer(4) } }, unit: None, representation: PureNumber })
    //         match f {
    //             Node::IntSwissKnife(node) => {
    //                 println!("{:#?}", node);
    //             }
    //             _ => {}
    //         });
    // println!("{:?}\n", f));
    set_value(&mut camera, "ExposureTime".to_string(), 1. * 1e6 / 30.0);
    // set_value(&mut camera, "AcquisitionFrameRate".to_string(), 300.0);
    // get_value(&mut camera, "DeviceLinkThroughputLimitMode".to_string());

    // Start streaming. Channel capacity is set to 3.
    let mut payload_rx = camera.start_streaming(3).unwrap();
    let (tx, rx): (Sender<Command>, Receiver<Command>) = mpsc::channel();

    let t0_first_thread = std::time::Instant::now();
    let t0_second_thread = std::time::Instant::now();

    thread::spawn(move || {
        let t0 = t0_first_thread;
        let mut t0_delta = std::time::Instant::now();
        let mut frame_counter = 0;
        let mut last_fps = [0.0; 100];
        let mut paused = false;

        // undistort image
        let width = 728;
        let height = 544;
        let left_margin = 100;
        let right_margin = 100;
        let top_margin = 0;
        let bottom_margin = 28;
        let min_x = -left_margin;
        let max_x = width as i32 + right_margin;
        let min_y = -top_margin;
        let max_y = height as i32 + bottom_margin;
        let new_width = (max_x - min_x) as u32;
        let new_height = (max_y - min_y) as u32;
        println!("old width: {}, old height: {}", width, height);
        println!("new width: {}, new height: {}", new_width, new_height);
        let precompute = matura::gen_table(width, height, new_width, new_height, min_x, min_y);
        let rl_comp = matura::compute_rl_coords::RLCompute::new();

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

        let mut last_command = Instant::now();
        let mut arduino_com = matura::arduino_com::ArduinoCom::new();

        let mut buffer = IMAGE_BUFFER_UNDISTORTED.lock().unwrap();
        buffer.0 = new_width;
        buffer.1 = new_height;
        drop(buffer);

        let mut undistorted_image = vec![0u8; (new_width * new_height * 3) as usize];
        let mut ball_comp = BallComp::new();
        let mut shoot_time = 0.;
        // whether the ball has already been shot at time shoot_time
        let mut shot = true;
        let mut time_since_catch = Instant::now();
        let mut pause_player = false;
        let mut moved_to_center = true;

        let mut player_calibration_positions = vec![];

        // functions
        const MIN_MOTOR: i32 = 0;
        const MAX_MOTOR: i32 = 400;
        const MIN_PIXEL: i32 = 226;
        const MAX_PIXEL: i32 = 350;
        fn move_y(
            x: f32,
            o_y: f32,
            arduino_com: &mut ArduinoCom,
            last_command: &mut Instant,
            rl_comp: &RLCompute,
            player_0: i32,
            player_target: &mut i32,
            paused_player: bool,
        ) {
            *player_target = o_y as i32;
            // let y = rl_comp.transform_point((x, o_y)).1 + player_0 as f32;
            let y = o_y;
            // println!("oy: {o_y}, y: {y}, 450 - y: {}", 450. - y);
            //
            // motor, pixel y
            // 0,   350
            // 330, 226
            //
            // pixel y, motor
            // 226, 330
            // 350, 0
            //
            // A(226, 330) B(350, 0)
            // y = mx + b
            // m = (y2 - y1) / (x2 - x1)
            // b = y1 - m * x1
            // m = (0 - 330) / (350 - 226) = -330 / 124 = -2.6612903226
            // b = 330 - (-2.66 * 226) = 330 + 600.76 = 931.4516129076
            // y = -2.66 * x + 930.76
            // motor = y - 350
            if y > MIN_PIXEL as f32
                && y < MAX_PIXEL as f32
                && last_command.elapsed().as_secs_f32() > 0.05
            {
                // convert from pixel y to motor
                let m = (MIN_MOTOR - MAX_MOTOR) as f32 / (MAX_PIXEL - MIN_PIXEL) as f32;
                let b = MAX_MOTOR as f32 - m * MIN_PIXEL as f32;
                let x = m * y + b;

                // new formula to convert from pixel y to motor
                // first convert y to f64, because the polynomial fit is done with f64 and it needs to be very precise
                let y = y as f64;
                let x = 0.0000601299 * y.powi(3) + -0.0499176837 * y.powi(2) + 10.8474743402 * y.powi(1) + -205.7294258740;
                let x = x as i32;
                // println!("y: {y}, x: {x}");

                // println!("sending: y: {x}");
                let paused_player = PAUSEPLAYER.load(Ordering::Relaxed);
                if !paused_player {
                    arduino_com.send_string(&format!("{}", x as i32));
                    *last_command = Instant::now();
                }
            }
        }
        fn move_center(
            arduino_com: &mut ArduinoCom,
            last_command: &mut Instant,
            player_0: i32,
            paused_player: bool,
        ) {
            if !PAUSEPLAYER.load(Ordering::Relaxed) {
                let y = (MIN_PIXEL + MAX_PIXEL) as f32 / 2.;
                if last_command.elapsed().as_secs_f32() > 0.05 {
                    // println!("Moving to center");
                    // arduino_com.send_string(&format!("{}", 212 as i32));
                    move_y(0., y, arduino_com, last_command, &RLCompute::new(), player_0, &mut 0, paused_player);
                    arduino_com.send_string(&"check 10".to_string());
                    *last_command = Instant::now();
                }
            }
        }

        loop {
            if let Ok(message) = rx.try_recv() {
                match message {
                    Exposure(value) => {
                        set_value(&mut camera, "ExposureTime".to_string(), value);
                    }
                    Start => {
                        paused = false;
                    }
                    Pause => {
                        paused = true;
                    }
                    Reset => {
                        arduino_com.send_string("full_reset");
                        let mut output = "".to_string();
                        while !output.starts_with("end") {
                            output = arduino_com.read_line();
                            // println!("f{:?}f", output.chars().collect::<Vec<char>>());
                            println!("{}", output);
                        }
                        println!("Finished full reset!");
                    }
                    ResetDC => {
                        arduino_com.send_string("reset_dc");
                    }
                    Stop => {
                        break;
                    }
                    ReloadRaw => {
                        let raw = load_raw();
                        let raw_image1 = raw
                            .as_raw()
                            .to_vec()
                            .chunks(4)
                            .map(|x| x[0..3].to_vec())
                            .flatten()
                            .collect::<Vec<u8>>();
                        let raw_image1 = raw_image1.as_slice();
                        // raw_image = raw_image1.clone();
                    }
                    MoveCenter => {
                        move_center(&mut arduino_com, &mut last_command, 0, pause_player);
                        moved_to_center = true;
                    }
                    Shoot => {
                        arduino_com.send_string("S");
                    }
                    PlayerCalibration(pos) => {
                        if pos == -1 {
                            println!("Player calibration started");
                            arduino_com.send_string("full_reset");
                            let mut output = "".to_string();
                            while !output.starts_with("end") {
                                output = arduino_com.read_line();
                                // println!("f{:?}f", output.chars().collect::<Vec<char>>());
                                println!("o: {}", output);
                            }
                            println!("Finished full reset!");
                        } else {
                            //     println!("Sending pos: {}", pos);
                            //     // arduino_com.send_string(&format!("{}", pos));
                            //     arduino_com.send_string("check 20");
                            //     std::thread::sleep(std::time::Duration::from_secs(2));
                            // arduino_com.read_everything();
                            arduino_com.send_string("I");
                            let output = arduino_com.read_line();
                            println!("o: {}", output);
                            // output format:    Pos: 32
                            let pos = output
                                .split_whitespace()
                                .nth(1)
                                .unwrap()
                                .parse::<f32>()
                                .unwrap();
                            println!("pos: {}", pos);
                            player_calibration_positions.push(pos as i32);
                        }
                    }
                    FinishPlayerCalibration(positions) => {
                        // todo
                        println!("Player calibration finished");
                        println!(
                            "player_calibration_positions: {:?}, len: {}\npositions: {:?}, len: {}",
                            player_calibration_positions,
                            player_calibration_positions.len(),
                            positions,
                            positions.len()
                        );
                        println!(
                            "positions = [{}]",
                            positions
                                .iter()
                                .enumerate()
                                .map(|x| format!(
                                    "[{}, {}]",
                                    player_calibration_positions[x.0], x.1
                                ))
                                .collect::<Vec<String>>()
                                .join(", ")
                        );
                        player_calibration_positions.clear();
                    }
                    _ => {
                        println!("Unknown command: {:?}", message);
                    }
                }
            }
            if paused {
                continue;
            }
            frame_counter += 1;
            // show rolling fps average
            let elapsed = t0.elapsed();
            let fps = frame_counter as f64 / elapsed.as_secs_f64();
            let delta = t0_delta.elapsed();
            t0_delta = std::time::Instant::now();
            if frame_counter % 1000 == 0 {
                // println!("avg fps: {:.2}", fps);
            }
            // let fps = 1.0 / delta.as_secs_f64()
            let fps = delta.as_secs_f64();
            last_fps[(frame_counter % last_fps.len()) as usize] = fps;
            *FPS.lock().unwrap() = last_fps.iter().sum::<f64>() / last_fps.len() as f64;
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
            // println!(
            //     "payload received! block_id: {:?}, timestamp: {:?}",
            //     payload.id(),
            //     payload.timestamp()
            // );
            let mut player_0 = 0;
            let mut player_target = 0;
            let mut current_player_pos = 0;
            if let Some(image_info) = payload.image_info() {
                // println!("{:?}\n", image_info);
                let width = image_info.width;
                let height = image_info.height;

                let image = payload.image();
                if let Some(image) = image {
                    // let old_height = height;
                    // let height = 700;
                    // let width = (width as f32 / old_height as f32 * height as f32) as u32;
                    // println!("Width: {}, Height: {}", width, height);
                    // let undistorted_image =
                    // undistort_image(&image, 30, width as u32, height as u32);
                    // buffer.2.clear();
                    // buffer.2.extend_from_slice(&undistorted_image);
                    matura::undistort_image_table(
                        image,
                        &mut undistorted_image,
                        &precompute,
                        new_width,
                        new_height,
                    );
                    let undistorted_clone = undistorted_image.clone();
                    let ball_comp_t0 = std::time::Instant::now();
                    subtract_image(&mut undistorted_image, raw_image);
                    // let u_image = DynamicImage::ImageRgb8(
                    // ImageBuffer::from_raw(new_width, new_height, undistorted_image.clone())
                    // .unwrap(),
                    // );
                    // u_image.save("undistorted_image.png");
                    // println!("subtracted image: {:?}", undistorted_image);
                    // std::process::exit(0);
                    let ball = matura::ball::find_ball(
                        undistorted_image.as_slice(),
                        new_width as u32,
                        new_height as u32,
                        &mut ball_comp,
                        elapsed.as_secs_f32(),
                    );
                    let elapsed_ball_comp = ball_comp_t0.elapsed();
                    if elapsed_ball_comp.as_secs_f32() * 1000.0 > 5. {
                        println!(
                            "warning: ball_comp: {:.2}ms",
                            elapsed_ball_comp.as_secs_f32() * 1000.0
                        );
                    }
                    // println!(
                    // "ball_comp: {:.2}ms",
                    // elapsed_ball_comp.as_secs_f32() * 1000.0
                    // );
                    player_target = ball.1 as i32;
                    let actual_player_pos = ACTUAL_PLAYER_POSITION.lock().unwrap().clone();
                    if !actual_player_pos.3
                        // && actual_player_pos.0 != 0
                        && actual_player_pos.1 != 0
                        && player_target != 0
                        && last_command.elapsed().as_secs_f32() > 0.1
                        && player_target as f32 > 236.
                        && (player_target as f32) < 345.
                    {
                        // so we know that the player is currently at actual_player_pos, but it thinks its at player_target
                        // example: ball: 450, actual: 400, target: 450 -> player_0 = 50
                        player_0 = player_target - actual_player_pos.1 as i32;
                        current_player_pos = actual_player_pos.1;
                        let mov =
                            ((player_target - actual_player_pos.1 as i32) as f32 * 2.35) as i32 / 3;
                        // println!("corrected position 0: actual_player_pos: {:?}, player_target: {}, player_0: {}, mov: {}", actual_player_pos, player_target, player_0, mov);
                        ACTUAL_PLAYER_POSITION.lock().unwrap().3 = true;
                        // arduino_com.send_string(&format!("{}", -mov as i32));
                        // last_command = Instant::now();
                    }

                    if ball_comp.velocity.x < 0.0 && ball_comp.velocity.magnitude() > 20. {
                        // the ball goes towards the goal
                        let intersection = ball_comp.intersection_x(44.);
                        if let Some(intersection) = intersection {
                            // println!("intersection: {:?}", intersection);
                            let t = intersection.1;
                            let prepone = 0.25;
                            if t > prepone && ball_comp.velocity.magnitude() > 50.0 {
                                // println!("t: {}, v: {}, t0: {}, t0 + t: {}", t, ball_comp.velocity.magnitude(), t0.elapsed().as_secs_f32(), t0.elapsed().as_secs_f32() + t);
                                shoot_time = t0.elapsed().as_secs_f32() + t - prepone;
                                shot = false;
                            }
                            // println!("t: {t}, shoot_time: {}", shoot_time);
                            let intersection = intersection.0;
                            if !FOLLOWBALL.load(Ordering::Relaxed) {
                                move_y(
                                    intersection.x,
                                    intersection.y,
                                    &mut arduino_com,
                                    &mut last_command,
                                    &rl_comp,
                                    player_0,
                                    &mut player_target,
                                    pause_player,
                                );
                            }
                            time_since_catch = Instant::now();
                            moved_to_center = true;
                        } else if time_since_catch.elapsed().as_secs_f32() > 0.2 && !moved_to_center {
                            if !FOLLOWBALL.load(Ordering::Relaxed) {
                                move_center(
                                    &mut arduino_com,
                                    &mut last_command,
                                    player_0,
                                    pause_player,
                                );
                            }
                            moved_to_center = true;
                        }
                    } else if time_since_catch.elapsed().as_secs_f32() > 0.2 && !moved_to_center {
                        if !FOLLOWBALL.load(Ordering::Relaxed) {
                            move_center(&mut arduino_com, &mut last_command, player_0, pause_player);
                        }
                        moved_to_center = true;
                    }

                    // shoot
                    if !shot && t0.elapsed().as_secs_f32() > shoot_time && !PAUSESHOOTING.load(Ordering::Relaxed)
                    && !PAUSEPLAYER.load(Ordering::Relaxed) {
                        arduino_com.send_string("S");
                        // println!("Shot!");
                        shot = true;
                    }

                    let y = ball.1 as f32;
                    if y > MIN_PIXEL as f32
                        && y < MAX_PIXEL as f32
                        && last_command.elapsed().as_secs_f32() > 0.05
                    {
                        // convert from pixel y to motor
                        let m = (MIN_MOTOR - MAX_MOTOR) as f32 / (MAX_PIXEL - MIN_PIXEL) as f32;
                        let b = MAX_MOTOR as f32 - m * MIN_PIXEL as f32;
                        let x = m * y + b;
                        // println!("sending: y: {y}");
                        // arduino_com.send_string(&format!("{}", x as i32));
                        // last_command = Instant::now();
                    }
                    if FOLLOWBALL.load(Ordering::Relaxed) {
                        move_y(
                            ball.0 as f32,
                            ball.1 as f32,
                            &mut arduino_com,
                            &mut last_command,
                            &rl_comp,
                            0,
                            &mut player_target,
                            pause_player,
                        );
                    }
                    if frame_counter % 1 == 0 {
                        // save to IMAGE_BUFFER
                        let mut buffer = IMAGE_BUFFER.lock().unwrap();
                        buffer.clear();
                        buffer.extend_from_slice(image);
                        drop(buffer);

                        let mut buffer = IMAGE_BUFFER_UNDISTORTED.lock().unwrap();
                        buffer.2.clear();
                        buffer.2.extend_from_slice(&undistorted_clone);
                    }
                }
            }
        }
    });

    // we dont need this thread anymore actually
    // thread::spawn(move || {
    //     let t0 = t0_second_thread;
    //     // this thread will poll the image all the time and check where the player is and save it to ACTUAL_PLAYER_POSITION
    //     let mut python_script = matura::detect_player::PythonScript::new();
    //     println!("Initialized python script");
    //     let mut last_t = Instant::now();
    //     loop {
    //         let t = Instant::now();
    //         let time = t0.elapsed().as_secs_f32();
    //         let buffer = { IMAGE_BUFFER_UNDISTORTED.lock().unwrap().clone() };
    //         let image = buffer.2;
    //         let width = buffer.0;
    //         let height = buffer.1;
    //         // convert the image to a DynamicImage
    //         let mut img = image::DynamicImage::ImageRgb8(
    //             image::ImageBuffer::from_raw(width as u32, height as u32, image.clone()).unwrap(),
    //         );
    //         for i in 0..width as usize * height as usize {
    //             let r = image[i * 3];
    //             let g = image[i * 3 + 1];
    //             let b = image[i * 3 + 2];
    //             img.put_pixel(
    //                 i as u32 % width as u32,
    //                 i as u32 / width as u32,
    //                 image::Rgba([r, g, b, 100]),
    //             );
    //         }
    //         // Get the dimensions of the image
    //         let (width, height) = img.dimensions();

    //         // Calculate the width of each slice
    //         let slice_width = width / 7;

    //         // Crop the left-most slice
    //         let left_slice = img.crop(
    //             0,
    //             (1. / 3. * height as f32) as u32,
    //             slice_width,
    //             (1. / 3. * height as f32) as u32,
    //         );
    //         let img = left_slice.resize_exact(128, 128, image::imageops::FilterType::Nearest);
    //         // get player position
    //         let values = img
    //             .to_rgb8()
    //             .into_raw()
    //             .iter()
    //             .map(|&v| v as u8)
    //             .collect::<Vec<u8>>();
    //         assert_eq!(values.len(), 128 * 128 * 3);
    //         // println!("prep: {:?}", t.elapsed());
    //         let player_position = python_script.detect_player(&values);
    //         let t = Instant::now();
    //         ACTUAL_PLAYER_POSITION.lock().unwrap().0 = player_position.0;
    //         ACTUAL_PLAYER_POSITION.lock().unwrap().1 =
    //             player_position.1 + (1. / 3. * height as f32) as u32;
    //         ACTUAL_PLAYER_POSITION.lock().unwrap().2 = time;
    //         ACTUAL_PLAYER_POSITION.lock().unwrap().3 = false;
    //         // println!("after: {:?}", t.elapsed());
    //         PLAYER_DETECTION_FPS.store(
    //             (1. / last_t.elapsed().as_secs_f32()) as i32,
    //             Ordering::Relaxed,
    //         );
    //         last_t = Instant::now();
    //     }
    // });

    run_camera_test(tx);
}

// x, y, time, used
pub static ACTUAL_PLAYER_POSITION: Mutex<(u32, u32, f32, bool)> = Mutex::new((0, 0, 0., false));
pub static PLAYER_DETECTION_FPS: AtomicI32 = AtomicI32::new(0);
pub static PAUSEPLAYER: AtomicBool = AtomicBool::new(false);
pub static PAUSESHOOTING: AtomicBool = AtomicBool::new(false);
pub static FOLLOWBALL: AtomicBool = AtomicBool::new(false);