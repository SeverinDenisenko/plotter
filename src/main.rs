#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, Theme};

use std::env;
use std::process::exit;
use egui::Vec2;

mod utils;
mod plots;
mod inputs;

use plots::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // If run without command line args display plotting window

        let options = eframe::NativeOptions {
            default_theme: Theme::Light,
            initial_window_size: Option::from(Vec2::new(600.0, 660.0)),
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

struct Plotter {
    // Plotting

    function: String,
    parametric1: String,
    parametric2: String,
    polar: String,

    lower_limit: String,
    higher_limit: String,
    intervals_amount: String,

    plot_provider: PlotProvider,
    plot_type: PlotType,

    // Data

    points: Vec<[f64; 2]>,
    are_data_computed: bool,

    // Window

    width: f32,
    height: f32,
}

impl Default for Plotter {
    fn default() -> Self {
        Self {
            function: "tan(x)".to_owned(),
            parametric1: "sin(t)".to_owned(),
            parametric2: "cos(t)".to_owned(),
            polar: "a".to_owned(),

            lower_limit: "0.0".to_owned(),
            higher_limit: "1.0".to_owned(),
            intervals_amount: "100".to_owned(),

            plot_provider: PlotProvider::Egui,
            plot_type: PlotType::Function2d,

            points: vec![],
            are_data_computed: false,

            width: 0.0,
            height: 0.0,
        }
    }
}

impl eframe::App for Plotter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.width = _frame.info().window_info.size.x;
            self.height = _frame.info().window_info.size.y;

            ui.horizontal(|ui| {
                Plotter::global_dark_light_mode_switch(ui);

                ui.add(egui::Separator::default().vertical());

                self.plotter_context_menu(ui);
                self.plot_context_menu(ui);
                self.utils_context_menu(ui);
                self.export_context_menu(ui);
            });

            ui.separator();

            self.input(ui);

            ui.separator();

            self.plot(ui);
        });
    }
}

impl Plotter {

    //////// Context menus ////////

    fn plotter_context_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Plotter", |ui| {
            ui.menu_button("Plot provider", |ui| {
                if ui.button("egui").clicked() {
                    self.plot_provider = PlotProvider::Egui;
                    ui.close_menu();
                }

                if ui.button("plotters").clicked() {
                    self.plot_provider = PlotProvider::Plotters;
                    ui.close_menu();
                }

                if ui.button("poloto").clicked() {
                    self.plot_provider = PlotProvider::Poloto;
                    ui.close_menu();
                }
            });

            if ui.button("Exit").clicked() {
                exit(0);
            }
        });
    }

    fn plot_context_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Plot", |ui| {
            ui.menu_button("Plot 2d", |ui| {
                if ui.button("Function").clicked() {
                    self.plot_type = PlotType::Function2d;
                    self.are_data_computed = false;
                    ui.close_menu();
                }

                if ui.button("Parametric").clicked() {
                    self.plot_type = PlotType::Parametric2d;
                    self.are_data_computed = false;
                    ui.close_menu();
                }

                if ui.button("Polar").clicked() {
                    self.plot_type = PlotType::Polar2d;
                    self.are_data_computed = false;
                    ui.close_menu();
                }

                if ui.button("Equation").clicked() {
                    self.plot_type = PlotType::Equation2d;
                    self.are_data_computed = false;
                    ui.close_menu();
                }

                if ui.button("High plot").clicked() {
                    self.plot_type = PlotType::High2d;
                    self.are_data_computed = false;
                    ui.close_menu();
                }
            });

            ui.menu_button("Data 2d", |ui| {
                if ui.button("Scatter").clicked() {
                    self.plot_type = PlotType::Scatter2d;
                    self.are_data_computed = false;
                    ui.close_menu();
                }

                if ui.button("Linear").clicked() {
                    self.plot_type = PlotType::Linear2d;
                    self.are_data_computed = false;
                    ui.close_menu();
                }

                if ui.button("Histogram").clicked() {
                    self.plot_type = PlotType::Histogram2d;
                    self.are_data_computed = false;
                    ui.close_menu();
                }
            });

            ui.menu_button("Plot 3d", |ui| {
                if ui.button("Parametric").clicked() {
                    self.plot_type = PlotType::Parametric3d;
                    self.are_data_computed = false;
                    ui.close_menu();
                }

                if ui.button("Equation").clicked() {
                    self.plot_type = PlotType::Equation3d;
                    self.are_data_computed = false;
                    ui.close_menu();
                }
            });
        });
    }

    fn utils_context_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Utils", |ui| {
            ui.menu_button("Calculus", |ui| {
                if ui.button("Curvature").clicked() {
                    //TODO
                    ui.close_menu();
                }

                if ui.button("Limit").clicked() {
                    //TODO
                    ui.close_menu();
                }

                if ui.button("Derivative").clicked() {
                    //TODO
                    ui.close_menu();
                }

                if ui.button("Integral").clicked() {
                    //TODO
                    ui.close_menu();
                }
            });

            ui.menu_button("Algebraic", |ui| {
                if ui.button("Solver").clicked() {
                    //TODO
                    ui.close_menu();
                }
            });
        });
    }

    fn export_context_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Export", |ui| {
            if ui.button("png").clicked() {
                //TODO
                ui.close_menu();
            }

            if ui.button("svg").clicked() {
                //TODO
                ui.close_menu();
            }
        });
    }

    fn global_dark_light_mode_switch(ui: &mut egui::Ui) {
        let style: egui::Style = (*ui.ctx().style()).clone();

        let new_visuals = style.visuals.light_dark_small_toggle_button(ui);

        if let Some(visuals) = new_visuals {
            ui.ctx().set_visuals(visuals);
        }
    }
}
