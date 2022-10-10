#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, Theme};

use std::env;
use egui::Vec2;

mod app;
mod plot_type;
mod compute;
mod plot;
mod inputs;
mod parser;

use app::*;
use plot_type::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // If run without command line args display plotting window

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
    } else {
        // Else do something else

        panic!("Don't implemented command line interaction.");
    }
}
