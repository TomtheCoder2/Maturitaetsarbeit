use eframe::Frame;
use egui::ColorImage;
use egui::Context;
use egui::TextureHandle;
use image::DynamicImage;
use image::Rgb;
use image::RgbImage;
use std::path::Path;
fn main() {
    start_gui();
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
struct Point {
    x: i32,
    y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
struct App {
    file_name: String,
    left_top: Point,
    right_top: Point,
    left_bottom: Point,
    right_bottom: Point,
    zoom_factor: f32,
}

impl Default for App {
    fn default() -> Self {
        App {
            file_name: "./java_ma/real_fiducial_sets/fiducial_1/topRight.png".to_string(),
            left_top: Point::default(),
            right_top: Point::default(),
            left_bottom: Point::default(),
            right_bottom: Point::default(),
            zoom_factor: 1.0,
        }
    }
}

fn load_texture_from_image(ctx: &Context, image: ColorImage) -> TextureHandle {
    ctx.load_texture("my_image", image, Default::default())
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("File name:");
                ui.text_edit_singleline(&mut self.file_name);
            });
            let img = match image::open(&self.file_name) {
                Ok(img) => img,
                Err(e) => {
                    ui.label(format!("Error: {}", e));
                    return;
                }
            };
            let img = img.to_rgb8();

            let width = img.width();
            let height = img.height();

            ui.horizontal(|ui| {
                ui.label("Zoom factor:");
                ui.add(egui::Slider::new(&mut self.zoom_factor, 0.1..=10.0));
            });

            macro_rules! gen_slider {
                ($name:ident) => {
                    ui.horizontal(|ui| {
                        ui.label(format!("{} x:", stringify!($name)));
                        ui.add(egui::Slider::new(&mut self.$name.x, 0..=width as i32));
                        // add spacer
                        ui.label("");
                        ui.label(format!("{} y:", stringify!($name)));
                        ui.add(egui::Slider::new(&mut self.$name.y, 0..=height as i32));
                    });
                    // ui.horizontal(|ui| {
                    //     ui.label(format!("{} y:", stringify!($name)));
                    //     ui.add(egui::Slider::new(&mut self.$name.y, 0..=height as i32));
                    // });
                };
            }

            gen_slider!(left_top);
            gen_slider!(right_top);
            gen_slider!(left_bottom);
            gen_slider!(right_bottom);

            // gen middle point
            let mut mid_x =
                (self.left_top.x + self.right_top.x + self.left_bottom.x + self.right_bottom.x) / 4;
            let mut mid_y =
                (self.left_top.y + self.right_top.y + self.left_bottom.y + self.right_bottom.y) / 4;
            ui.horizontal(|ui| {
                ui.label("Middle x:");
                ui.add(egui::Slider::new(&mut mid_x, 0..=width as i32));
                ui.label("Middle y:");
                ui.add(egui::Slider::new(&mut mid_y, 0..=height as i32));
            });

            if ui.button("Save").clicked() {
                // save the image at the same location but change the file name to 0_midx_midy.png
                let path = Path::new(&self.file_name);
                let mut path = path.to_path_buf();
                path.set_file_name(format!(
                    "{}_{}.png",
                    mid_x.max(0).min(width as i32),
                    mid_y.max(0).min(height as i32)
                ));
                image::save_buffer(
                    &path,
                    &img,
                    width as u32,
                    height as u32,
                    image::ColorType::Rgb8,
                );
            }

            let draw_circle = |image: &mut RgbImage, x: i32, y: i32, color: [u8; 3]| {
                for i in x - 5..x + 5 {
                    for j in y - 5..y + 5 {
                        if (i - x).pow(2) + (j - y).pow(2) <= 25 {
                            // check that the point is inside the image
                            if i < 0 || i >= width as i32 || j < 0 || j >= height as i32 {
                                continue;
                            }
                            image.put_pixel(i as u32, j as u32, Rgb(color));
                        }
                    }
                }
            };

            // draw the points on the image
            let mut image = RgbImage::new(width, height);
            // copy the pixels from img
            for x in 0..width {
                for y in 0..height {
                    let pixel = img.get_pixel(x, y);
                    image.put_pixel(x, y, *pixel);
                }
            }
            draw_circle(&mut image, self.left_top.x, self.left_top.y, [255, 0, 0]);
            draw_circle(&mut image, self.right_top.x, self.right_top.y, [0, 255, 0]);
            draw_circle(
                &mut image,
                self.left_bottom.x,
                self.left_bottom.y,
                [0, 0, 255],
            );
            draw_circle(
                &mut image,
                self.right_bottom.x,
                self.right_bottom.y,
                [255, 255, 0],
            );
            // draw the middle point
            for x in mid_x.max(5) - 5..mid_x + 5 {
                for y in mid_y.max(5) - 5..mid_y + 5 {
                    image.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
                }
            }
            // draw the lines
            let mut image = image.into_raw();
            let mut draw_line = |start: Point, end: Point| {
                let start = start;
                let end = end;
                let dx = end.x - start.x;
                let dy = end.y - start.y;
                let m = dy as f64 / dx as f64;
                let c = start.y as f64 - m * start.x as f64;
                for x in start.x..end.x {
                    let y = (m * x as f64 + c) as i32;
                    let y = y.max(0).min(height as i32 - 1);
                    let x = x.max(0).min(width as i32 - 1);
                    let idx = (y * width as i32 + x) as usize * 3;
                    image[idx] = 255;
                    image[idx + 1] = 0;
                    image[idx + 2] = 0;
                }
            };
            // draw the cross
            draw_line(self.left_top, self.right_bottom);
            draw_line(self.right_top, self.left_bottom);

            // let image = ColorImage::from_rgb([width as usize, height as usize], &image);
            let image = DynamicImage::ImageRgb8(
                RgbImage::from_raw(width as u32, height as u32, image).unwrap(),
            );
            let new_width = width * self.zoom_factor as u32;
            let new_height = height * self.zoom_factor as u32;
            let image =
                image.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);
            // convert to ColorImage
            let image =
                ColorImage::from_rgb([new_width as usize, new_height as usize], image.as_bytes());
            let texture = load_texture_from_image(ctx, image.clone());
            ui.add(
                egui::Image::new(&texture), // .max_width(200.0).rounding(10.0)
            );
        });
    }

    /// Called by the frame work to save state before shutdown.
    /// On Windows its saved here: C:\Users\UserName\AppData\Roaming\Phoenix\data\app.ron
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        println!("Saving state");
        // self.version = VERSION.to_string();
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = App::default();
        if let Some(storage) = cc.storage {
            app = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        };
        app
    }
}

pub fn start_gui() {
    println!("Starting Camera Test GUI");
    eframe::run_native(
        "Find fiducial coords",
        Default::default(),
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .expect("Failed to run Camera Test");
}
