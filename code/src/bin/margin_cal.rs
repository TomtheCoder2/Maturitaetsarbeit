use eframe::Frame;
use egui::ColorImage;
use egui::Context;
use egui::TextureHandle;
use image::Rgb;
use image::RgbImage;

fn main() {
    start_gui();
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
struct App {
    number: i32,
    left_margin: i32,
    right_margin: i32,
    top_margin: i32,
    bottom_margin: i32,
}

impl Default for App {
    fn default() -> Self {
        App {
            number: 7,
            left_margin: 70,
            right_margin: 70,
            top_margin: 10,
            bottom_margin: 10,
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
                // ui.label("Hello World!");
                ui.horizontal(|ui| {
                    ui.label("Number:");
                    ui.add(egui::DragValue::new(&mut self.number));
                });
                ui.horizontal(|ui| {
                    ui.label("Left margin:");
                    ui.add(egui::DragValue::new(&mut self.left_margin));
                });
                ui.horizontal(|ui| {
                    ui.label("Right margin:");
                    ui.add(egui::DragValue::new(&mut self.right_margin));
                });
                ui.horizontal(|ui| {
                    ui.label("Top margin:");
                    ui.add(egui::DragValue::new(&mut self.top_margin));
                });
                ui.horizontal(|ui| {
                    ui.label("Bottom margin:");
                    ui.add(egui::DragValue::new(&mut self.bottom_margin));
                });
            });
            let number = self.number;
            let img = match image::open(format!("input_images/input_image{}.png", number)) {
                Ok(img) => img,
                Err(e) => {
                    ui.label(format!("Error: {}", e));
                    return;
                }
            };
            let img = img.to_rgb8();

            let width = img.width();
            let height = img.height();
            let min_x = -self.left_margin;
            let max_x = width as i32 + self.right_margin;
            let min_y = -self.top_margin;
            let max_y = height as i32 + self.bottom_margin;
            let new_width = (max_x - min_x) as u32;
            let new_height = (max_y - min_y) as u32;
            // println!("old width: {}, old height: {}", width, height);
            // println!("new width: {}, new height: {}", new_width, new_height);
            let precompute = matura::gen_table(width, height, new_width, new_height, min_x, min_y);
            let mut image = vec![0u8; (new_width * new_height * 3) as usize];
            matura::undistort_image_table(&img, &mut image, &precompute, new_width, new_height);

            if ui.button("Save").clicked() {
                let mut final_image = RgbImage::new(new_width, new_height);
                for x in 0..new_width {
                    for y in 0..new_height {
                        let pixel_index = (y * new_width + x) as usize * 3;
                        let pixel = &image[pixel_index..pixel_index + 3];
                        final_image.put_pixel(x, y, Rgb([pixel[0], pixel[1], pixel[2]]));
                    }
                }
                final_image
                    .save(format!("output_images/output{}.png", number))
                    .expect("Failed to save image");
            }

            let image = ColorImage::from_rgb([new_width as usize, new_height as usize], &image);
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
        "Margin adjustment",
        Default::default(),
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .expect("Failed to run Camera Test");
}
