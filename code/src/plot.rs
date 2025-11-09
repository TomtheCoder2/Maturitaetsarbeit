// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// #![expect(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints, PlotUi, Points};

#[derive(Default)]
pub struct PlotApp {
    pub insert_order: bool,
    pub graph: Vec<[f64; 2]>,
    pub graph2: Vec<[f64; 2]>,
    pub graph3: Vec<[f64; 2]>,
}

impl eframe::App for PlotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("If checked the legend will follow the order as the curves are inserted");
            ui.checkbox(&mut self.insert_order, "Insert order");

            fn add_plot_points(name: String, points: Vec<[f64; 2]>,plot_ui: &mut PlotUi) {
                plot_ui.line(Line::new(
                    name.clone(),
                    PlotPoints::from(points.clone()),
                ));
                plot_ui.points(Points::new(
                    format!("{} points", name),
                    PlotPoints::from(points.clone()),
                ));
            }

            Plot::new("My Plot")
                .legend(Legend::default().follow_insertion_order(self.insert_order))
                .show(ui, |plot_ui| {
                    add_plot_points("Graph 1".to_string(), self.graph.clone(), plot_ui);
                    add_plot_points("Graph 2".to_string(), self.graph2.clone(), plot_ui);
                    add_plot_points("Graph 3".to_string(), self.graph3.clone(), plot_ui);
                });
            // Remember the position of the plot
        });
    }
}