use eframe::egui;
use egui_extras::RetainedImage;

// fn main() {
//     let mut arduino_com = matura::arduino_com::ArduinoCom::new();
//     arduino_com.send_string("full_reset");
//     let mut output = "".to_string();
//     while !output.starts_with("end") {
//         output = arduino_com.read_line();
//         // println!("f{:?}f", output.chars().collect::<Vec<char>>());
//         println!("{}", output);
//     }
//     println!("Finished full reset!");
//     // sleep 1 second
//     std::thread::sleep(std::time::Duration::from_secs(2));
//     let pos = [50, 100, 150, 200, 250, 300, 350];
//     for p in pos.iter() {
//         println!("Sending pos: {}", p);
//         arduino_com.send_string(&format!("{}", p));
//         arduino_com.send_string("check 20");
//         std::thread::sleep(std::time::Duration::from_secs(2));
//         arduino_com.send_string("I");
//         output = arduino_com.read_line();
//         println!("o: {}", output);
//         std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
//     }
// }

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Image Click Demo",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    image: Option<RetainedImage>,
    click_position: Option<egui::Pos2>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            image: None,
            click_position: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Click on the image to draw a red dot!");

            if let Some(image) = &self.image {
                let texture = image.texture_id(ctx);
                let size = image.size_vec2();

                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let response = ui.image(texture, size).interact(egui::Sense::click());

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
            } else {
                ui.label("Loading image...");
                if self.image.is_none() {
                    if let Ok(image) = RetainedImage::from_image_path("example.png") {
                        self.image = Some(image);
                    } else {
                        ui.label("Failed to load the image. Make sure 'example.png' is in the project directory.");
                    }
                }
            }
        });
    }
}
