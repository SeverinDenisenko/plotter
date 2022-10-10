use eframe::{egui, Theme};
use egui::Vec2;
use crate::app::*;

pub fn run_default(){
    let options = eframe::NativeOptions {
        default_theme: Theme::Light,
        initial_window_size: Option::from(Vec2::new(900.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Plotter",
        options,
        Box::new(|_cc| Box::new(Plotter::default())),
    );
}