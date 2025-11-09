use crate::ball::{standard_selection, MAGNITUE_DIFF};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::mpsc::{Sender};
use std::sync::{mpsc, Mutex};
use std::time::Instant;

use cameleon::u3v::{ControlHandle, StreamHandle};
use cameleon::Camera;
use eframe::{Frame};
use egui::{Color32, Context};
use egui::{ColorImage, TextureHandle};
use crate::ball::BallComp;
use crate::{ball, increment_last_number_in_filename};
use std::sync::atomic::Ordering;
use atomic_float::AtomicF32;

use crate::live_feed::Command::*;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer};
use serde::{Deserialize, Serialize};
use crate::cam_thread::{load_raw, CamThread};
use crate::live_feed::Command::{ReloadRaw};

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
    SelectionFn {
        selection_type: Selection,
        r: u8,
        g: u8,
        b: u8,
        sum: i32,
    },
    Radius {
        min_radius: f32,
        max_radius: f32,
    },
    BallPixel {
        min_pixel: i32,
        max_pixel: i32,
    },
    PlayerXCoord(i32)
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Selection {
    Separation,
    Addition,
}

#[derive(Debug)]
pub enum Mode {
    Normal,
    PlayerCalibration,
}

#[derive(Serialize, Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    file_name: String,
    #[serde(skip)]
    pub sender: Sender<Command>,
    exposure: f64,
    auto_exposure: bool,
    calibration_mode: bool,
    brightness: f64,
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
    show_undistorted: bool,
    overlay_raw_img: bool,
    #[serde(skip)]
    ball_comp: BallComp,
    #[serde(skip)]
    start_time: Instant,
    // #[serde(skip)]
    // compute_rl_coords: compute_rl_coords::RLCompute,
    #[serde(skip)]
    last_command: Instant,
    speed: i32,
    motor_pos: i32,
    #[serde(skip)]
    last_frame: Instant,
    overlay_ball: bool,
    overlay_ball_prediction: bool,
    show_player_predicition: bool,
    show_min_max_pixel: bool,
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
    follow_ball: bool,
    selection: Selection,
    r: u8,
    g: u8,
    b: u8,
    last_rgb_sum: (u8, u8, u8, i32),
    sum: i32,
    #[serde(skip)]
    selection_fn: Box<dyn Fn(u8, u8, u8) -> bool>,
    show_selection: bool,
    // maybe convert to atomic
    min_radius: f32,
    max_radius: f32,
    min_pixel: i32,
    max_pixel: i32,
    timing_offset: f32,
    magnitude_diff: i32,
    player_x_coord: i32,
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
            brightness: 1.0,
            last_cal_image: Instant::now(),
            calibration_interval: 0.5,
            paused: false,
            recorded_images: Vec::new(),
            recording: false,
            // read the image from ./raw.png
            raw_image: load_raw(),
            show_original: false,
            show_undistorted: true,
            overlay_raw_img: false,
            ball_comp: BallComp::default(),
            start_time: Instant::now(),
            // compute_rl_coords: compute_rl_coords::RLCompute::new(),
            last_command: Instant::now(),
            speed: 100,
            motor_pos: 0,
            last_frame: Instant::now(),
            overlay_ball: true,
            overlay_ball_prediction: true,
            show_player_predicition: true,
            show_min_max_pixel: false,
            pause_player: false,
            pause_shooting: false,
            click_position: None,
            mode: Mode::Normal,
            player_calibration_pos: 0,
            final_player_calibration_positions: Vec::new(),
            player_calibration_message: "".to_string(),
            follow_ball: false,
            selection: Selection::Addition,
            r: 30,
            g: 30,
            b: 30,
            sum: 5 * 30,
            last_rgb_sum: (30, 30, 30, 5 * 30),
            selection_fn: Box::new(standard_selection),
            show_selection: false,
            min_radius: 10.,
            max_radius: 20.,
            min_pixel: 226,
            max_pixel: 350,
            timing_offset: 0.25,
            magnitude_diff: 20,
            player_x_coord: 44,
        }
    }
}


fn load_texture_from_image(ctx: &Context, image: ColorImage) -> TextureHandle {
    ctx.load_texture("my_image", image, Default::default())
}



pub static IMAGE_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());
pub(crate) static IMAGE_BUFFER_UNDISTORTED: Mutex<(u32, u32, Vec<u8>)> = Mutex::new((0, 0, Vec::new()));
pub static FPS: Mutex<f64> = Mutex::new(0.0);

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
            let comp_fps = 1. / self.last_frame.elapsed().as_secs_f64();
            self.last_frame = Instant::now();
            let player_detection_fps = PLAYER_DETECTION_FPS.load(Ordering::Relaxed);
            ui.label(format!("Comp FPS: {:>5.0}, player detection fps: {}", comp_fps, player_detection_fps));
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
            let ball = ball::read_image_vis(&mut subtracted_image, &mut original_undistorted_image, &mut self.ball_comp, time, &self.selection_fn, self.min_radius, self.max_radius, self.overlay_ball_prediction);
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
            

            // let ball_irl = self.compute_rl_coords.transform_point((ball.0 as f32, ball.1 as f32));
            let ball_irl = (ball.0 as f32, ball.1 as f32);
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
            
            for pixel in image.as_mut_rgb8().unwrap().pixels_mut() {
                pixel.0[0] = (pixel.0[0] as f64 * self.brightness).min(255.0) as u8;
                pixel.0[1] = (pixel.0[1] as f64 * self.brightness).min(255.0) as u8;
                pixel.0[2] = (pixel.0[2] as f64 * self.brightness).min(255.0) as u8;
            }

            if self.show_selection {
                match self.selection {
                    Selection::Separation => {
                        // go through each pixel and check if they have r, g, b > self.r, self.g, self.b
                        // then calculate which color is the most over the threshold and color the pixel with that color
                        for x in 0..image.width() {
                            for y in 0..image.height() {
                                let pixel = subtracted_image.get_pixel(x, y);
                                let r = pixel[0];
                                let g = pixel[1];
                                let b = pixel[2];
                                if (self.selection_fn)(r, g, b) {
                                    let max = r.max(g).max(b);
                                    let color = if max == r {
                                        image::Rgba([255, 0, 0, 255])
                                    } else if max == g {
                                        image::Rgba([0, 255, 0, 255])
                                    } else {
                                        image::Rgba([0, 0, 255, 255])
                                    };
                                    image.put_pixel(x, y, color);
                                }
                            }
                        }
                    }
                    Selection::Addition => {
                        // go through each pixel and check if they have r, g, b > self.r, self.g, self.b
                        // then calculate which color is the most over the threshold and color the pixel with that color
                        for x in 0..image.width() {
                            for y in 0..image.height() {
                                let pixel = subtracted_image.get_pixel(x, y);
                                let r = pixel[0];
                                let g = pixel[1];
                                let b = pixel[2];
                                if (self.selection_fn)(r, g, b) {
                                    image.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
                                }
                            }
                        }
                    }
                }
            }

            let min_pixel_color = Color32::from_rgb(255, 165, 0);
            let max_pixel_color = Color32::from_rgb(0, 255, 0);
            let player_x_coord_color = Color32::from_rgb(0, 0, 255);
            // if show min max pixel, draw a line in the color min_pixeL_color at y = min_pixel and max_pixel with max_pixel_color
            if self.show_min_max_pixel {
                for x in 0..image.width() {
                    image.put_pixel(x, self.min_pixel as u32, image::Rgba(min_pixel_color.to_array()));
                    image.put_pixel(x, self.max_pixel as u32, image::Rgba(max_pixel_color.to_array()));
                }
                for y in 0..image.height() {
                    image.put_pixel(self.player_x_coord as u32, y, image::Rgba(player_x_coord_color.to_array()));
                }
            }

            let ci_image = ColorImage::from_rgb(
                [image.width() as usize, image.height() as usize],
                image.as_bytes(),
            );

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
                    save_img(original_image.clone(), self.file_name.clone() + "_raw");
                    if self.calibration_mode {
                        self.file_name = increment_last_number_in_filename(&self.file_name)
                            .expect("Could not increment last number in filename");
                        self.last_cal_image = Instant::now();
                    }
                }
                if ui.button("Save Raw overlay").clicked() {
                    save_img(unmodified_original_undistorted_image.clone(), "./raw.png".to_string());
                    self.raw_image = load_raw();
                    self.sender.send(ReloadRaw).unwrap();
                }
                // ui.label("Speed:");
                // ui.add(egui::Slider::new(&mut self.speed, 0..=1000));
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
                        let prefix = format!(
                            "./recording_{}/",
                            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
                        );
                        std::fs::create_dir_all(&prefix).unwrap();
                        for (i, image) in self.recorded_images.iter().enumerate() {
                            let mut file_name = prefix.clone();
                            file_name.push_str(&format!("{:04}.png", i));
                            // let mut file = std::fs::File::create(&file_name).unwrap();
                            // write image to file
                            let mut image_buffer = ImageBuffer::new(width, height);
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

            ui.horizontal(|ui| {
                ui.label("Image: ");
                ui.label("Exposure: ");
                if ui
                    .add(egui::DragValue::new(&mut self.exposure).speed(0.01))
                    .clicked()
                    || ui.button("Set Exposure").clicked()
                {
                    self.sender.send(Command::Exposure(self.exposure)).unwrap();
                }
                ui.label("Brightness: ");
                ui.add(egui::DragValue::new(&mut self.brightness).speed(0.01).range(0.0..=10.0));
                ui.checkbox(&mut self.show_original, "Show original Image");
                ui.checkbox(&mut self.show_undistorted, "Show undistorted Image");
                ui.checkbox(&mut self.auto_exposure, "Auto Exposure");
                ui.checkbox(&mut self.calibration_mode, "Calibration Mode");
                ui.checkbox(&mut self.overlay_raw_img, "Overlay Raw Image");
            });
            ui.horizontal(|ui| {
                ui.label("Ball Detection: ");
                ui.label("Min Radius:");
                if ui.add(egui::DragValue::new(&mut self.min_radius).speed(0.1)).changed() {
                    self.sender.send(Command::Radius { min_radius: self.min_radius, max_radius: self.max_radius }).unwrap();
                }
                ui.label("Max Radius:");
                if ui.add(egui::DragValue::new(&mut self.max_radius).speed(0.1)).changed() {
                    self.sender.send(Command::Radius { min_radius: self.min_radius, max_radius: self.max_radius }).unwrap();
                }
                ui.colored_label(min_pixel_color,"Min Pixel:");
                if ui.add(egui::DragValue::new(&mut self.min_pixel).speed(0.1)).changed() {
                    self.sender.send(Command::BallPixel { min_pixel: self.min_pixel, max_pixel: self.max_pixel }).unwrap();
                }
                ui.colored_label(max_pixel_color,"Max Pixel:");
                if ui.add(egui::DragValue::new(&mut self.max_pixel).speed(0.1)).changed() {
                    self.sender.send(Command::BallPixel { min_pixel: self.min_pixel, max_pixel: self.max_pixel }).unwrap();
                }
                ui.checkbox(&mut self.show_min_max_pixel, "Show min max pixel");

                // color stuff
                egui::ComboBox::from_label("Selection")
                    .selected_text(format!("{}", match self.selection {
                        Selection::Separation => "Separation",
                        Selection::Addition => "Addition",
                    }))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selection, Selection::Separation, "Separation");
                        ui.selectable_value(&mut self.selection, Selection::Addition, "Addition");
                    });
                match self.selection {
                    Selection::Separation => {
                        // inputs for r g b
                        ui.label("R:");
                        ui.add(egui::DragValue::new(&mut self.r).speed(1));
                        ui.label("G:");
                        ui.add(egui::DragValue::new(&mut self.g).speed(1));
                        ui.label("B:");
                        ui.add(egui::DragValue::new(&mut self.b).speed(1));
                        let r = self.r; // Clone current values of self.r, self.g, self.b
                        let g = self.g;
                        let b = self.b;

                        // Create a closure with `'static` lifetime
                        self.selection_fn = Box::new(move |r_in, g_in, b_in| {
                            r_in > r && g_in > g && b_in > b
                        });
                    }
                    Selection::Addition => {
                        ui.label("Sum:");
                        ui.add(egui::DragValue::new(&mut self.sum).speed(1));
                        let sum = self.sum; // Clone current value of self.sum

                        // Create a closure with `'static` lifetime
                        self.selection_fn = Box::new(move |r_in, g_in, b_in| {
                            r_in as i32 + g_in as i32 + b_in as i32 > sum
                        });
                    }
                }
                if self.last_rgb_sum != (self.r, self.g, self.b, self.sum) {
                    self.last_rgb_sum = (self.r, self.g, self.b, self.sum);
                    self.sender.send(Command::SelectionFn { selection_type: self.selection, r: self.r, g: self.g, b: self.b, sum: self.sum }).unwrap();
                }
                ui.checkbox(&mut self.show_selection, "Show Selection");
                ui.checkbox(&mut self.overlay_ball, "Overlay Ball Detection");
                ui.checkbox(&mut self.overlay_ball_prediction, "Overlay Ball Prediction");
                // ui.checkbox(&mut self.show_player_predicition, "Show Player Prediction");
            });
            ui.horizontal(|ui| {
                ui.label("\t");
                ui.label("Magnitude Diff:");
                if ui.add(egui::DragValue::new(&mut self.magnitude_diff).speed(1)).changed() {
                    MAGNITUE_DIFF.store(self.magnitude_diff, Ordering::Relaxed);
                }
                ui.colored_label(player_x_coord_color,"Player X coord:");
                if ui.add(egui::DragValue::new(&mut self.player_x_coord).speed(1.)).changed() {
                    self.sender.send(Command::PlayerXCoord(self.player_x_coord)).unwrap();
                }
            });
            ui.horizontal(|ui|{
                ui.label("Player:");
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
                ui.label("Shoot prepone time:");
                if ui.add(egui::DragValue::new(&mut self.timing_offset).speed(0.1)).changed() {
                    TIMING_OFFSET.store(self.timing_offset, Ordering::Relaxed);
                }
                ui.label("s");
                if ui.checkbox(&mut self.pause_player, "Pause player movement").clicked() {
                    println!("Pause player movement: {}", self.pause_player);
                    PAUSEPLAYER.store(self.pause_player, Ordering::Relaxed);
                    println!("atomic: Pause player movement: {}", PAUSEPLAYER.load(Ordering::Relaxed));
                }
                if ui.checkbox(&mut self.pause_shooting, "Pause shooting").clicked() {
                    PAUSESHOOTING.store(self.pause_shooting, Ordering::Relaxed);
                }
                if ui.checkbox(&mut self.follow_ball, "Follow Ball").clicked() {
                    FOLLOWBALL.store(self.follow_ball, Ordering::Relaxed);
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
                            self.sender.send(PlayerCalibration(POS[self.player_calibration_pos as usize])).unwrap();
                            if self.player_calibration_pos >= POS.len() as i32 - 1 {
                                self.mode = Mode::Normal;
                                println!("Player calibration finished");
                                println!("pos: {:?}", self.final_player_calibration_positions);
                                println!("Player calibration positions: {}", self.final_player_calibration_positions.iter().enumerate().map(|(i, x)| format!("\t{}: {}: {}\n", i, POS[i], x)).collect::<Vec<String>>().join(""));
                                self.sender.send(FinishPlayerCalibration(self.final_player_calibration_positions.clone())).unwrap();
                                PAUSEPLAYER.store(self.pause_player, Ordering::Relaxed);
                                self.pause_player = false;
                            }
                        }
                    }
                } else if ui.button("Player Calibration").clicked() {
                    self.mode = Mode::PlayerCalibration;
                    self.player_calibration_pos = -1;
                    PAUSEPLAYER.store(true, Ordering::Relaxed);
                    self.pause_player = true;
                    self.sender.send(Command::PlayerCalibration(-1)).unwrap();
                    self.final_player_calibration_positions.clear();
                }
                if self.calibration_mode {
                    ui.label("Calibration Image Interval: ");
                    ui.add(egui::DragValue::new(&mut self.calibration_interval).speed(0.1));
                }
            });

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
            // ui.image(&texture);
            // ui.centered_and_justified(|ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    if self.show_original {
                        ui.add(
                            egui::Image::new(&original_texture), // .max_width(200.0).rounding(10.0)
                        );
                    }
                    if self.show_undistorted {
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
                    }
                });
            });
            // });
            ui.label(format!("FPS: {:>5.0}", 1. / (*FPS.lock().unwrap())));
        });
        ctx.request_repaint();
    }

    /// Called by the frame work to save state before shutdown.
    /// On Windows its saved here: C:\Users\UserName\AppData\Roaming\Phoenix\data\app.ron
    /// On MacOS its saved here: ~/Library/Application Support/Live-Feed/app.ron
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        println!("Saving...");
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
        FOLLOWBALL.store(app.follow_ball, Ordering::Relaxed);
        PAUSESHOOTING.store(app.pause_shooting, Ordering::Relaxed);
        app.sender
            .send(Command::Radius {
                min_radius: app.min_radius,
                max_radius: app.max_radius,
            })
            .unwrap();
        app.sender
            .send(Command::BallPixel {
                min_pixel: app.min_pixel,
                max_pixel: app.max_pixel,
            })
            .unwrap();
        app.sender
            .send(Command::SelectionFn {
                selection_type: app.selection,
                r: app.r,
                g: app.g,
                b: app.b,
                sum: app.sum,
            })
            .unwrap();
        if app.paused {
            app.sender.send(Command::Pause).unwrap();
        } else {
            app.sender.send(Command::Start).unwrap();
        }
        app.sender.send(Command::PlayerXCoord(app.player_x_coord)).unwrap();
        app
    }
}

pub fn run_gui(tx: Sender<Command>) {
    println!("Starting Camera GUI");
    eframe::run_native(
        "Live Feed",
        Default::default(),
        Box::new(|cc| Ok(Box::new(App::new(tx, cc)))),
    )
        .expect("Failed to run Camera");
}

pub fn get_value(camera: &mut Camera<ControlHandle, StreamHandle>, name: String) {
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
            let value_value = value.value(&params_ctxt).clone();
            let name = value.as_node().name(&params_ctxt);

            println!("{}: {:?}", name, value_value);
        }
    } else {
        println!("{name} is not readable");
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

pub fn main() {
    let cam_thread = CamThread::new();
    let tx = cam_thread.start();

    run_gui(tx);
}

// x, y, time, used
pub static PLAYER_DETECTION_FPS: AtomicI32 = AtomicI32::new(0);
pub static PAUSEPLAYER: AtomicBool = AtomicBool::new(false);
pub static PAUSESHOOTING: AtomicBool = AtomicBool::new(false);
pub static FOLLOWBALL: AtomicBool = AtomicBool::new(false);
pub static TIMING_OFFSET: AtomicF32 = AtomicF32::new(0.);
