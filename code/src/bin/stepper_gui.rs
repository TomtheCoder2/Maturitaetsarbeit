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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
struct App {
    #[serde(skip)]
    arduino_com: matura::arduino_com::ArduinoCom,
    stepper_abs_motor_pos: i32,
    stepper_rel_motor_pos: i32,
    speed: i32,
}

impl Default for App {
    fn default() -> Self {
        App {
            arduino_com: matura::arduino_com::ArduinoCom::new(),
            stepper_abs_motor_pos: 0,
            stepper_rel_motor_pos: 0,
            speed: 100,
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
                ui.label("Stepper abs motor pos:");
                ui.add(egui::Slider::new(&mut self.stepper_abs_motor_pos, 0..=140));
            });
            ui.horizontal(|ui| {
                ui.label("Stepper motor rel pos:");
                ui.add(egui::Slider::new(
                    &mut self.stepper_rel_motor_pos,
                    -140..=140,
                ));
            });
            ui.horizontal(|ui| {
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.speed, 0..=1000));
            });
            if ui.button("Send abs").clicked() {
                self.arduino_com.send_stepper_motor_speed(self.speed);
                self.arduino_com
                    .send_stepper_motor_pos(self.stepper_abs_motor_pos);
            }
            if ui.button("Send rel").clicked() {
                self.arduino_com.send_stepper_motor_speed(self.speed);
                self.stepper_abs_motor_pos += self.stepper_rel_motor_pos;
                self.arduino_com
                    .send_stepper_motor_pos(self.stepper_abs_motor_pos);
            }
            if ui.button("Reset").clicked() {
                self.arduino_com.send_stepper_motor_speed(self.speed);
                self.stepper_abs_motor_pos = 0;
                self.arduino_com
                    .send_command(com::commands::Command::Reset(0));
            }
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
        let mut app;
        if let Some(storage) = cc.storage {
            app = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        } else {
            app = App::default();
        }
        app.arduino_com.send_stepper_motor_speed(app.speed);
        app
    }
}

pub fn start_gui() {
    println!("Starting Stepper Motor GUI");
    eframe::run_native(
        "Stepper Motor GUI",
        Default::default(),
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .expect("Failed to run Camera Test");
}
