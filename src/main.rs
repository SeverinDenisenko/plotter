#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, Theme};
use egui_extras::{Size, StripBuilder};

use std::env;
use std::process::exit;
use egui::Vec2;

mod types;
mod compute;
mod plots;
mod inputs;

use types::*;

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

struct Plotter {
    // Plotting
    plots: Vec<PlotItem>,
    plot_provider: PlotProvider,

    // Window

    width: f32,
    height: f32,
}

impl Default for Plotter {
    fn default() -> Self {
        Self {
            plots: vec![PlotItem::default_function()],
            plot_provider: PlotProvider::Egui,

            width: 0.0,
            height: 0.0,
        }
    }
}

impl eframe::App for Plotter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("context_menu")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(3.0);

                ui.horizontal(|ui| {
                    Plotter::global_dark_light_mode_switch(ui);

                    self.add_context_menu(ui);
                    self.utils_context_menu(ui);
                    self.plotter_context_menu(ui);
                    self.export_context_menu(ui);
                });

                ui.add_space(0.5);

            });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                self.input_all(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.width = _frame.info().window_info.size.x;
            self.height = _frame.info().window_info.size.y;

            StripBuilder::new(ui)
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        self.plot_all(ui);
                    });
                });
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

    fn add_context_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Add", |ui| {
            ui.menu_button("Plot 2d", |ui| {
                if ui.button("Function").clicked() {
                    self.plots.push(PlotItem::default_function());
                    ui.close_menu();
                }

                if ui.button("Parametric").clicked() {
                    self.plots.push(PlotItem::default_parametric());
                    ui.close_menu();
                }

                if ui.button("Polar").clicked() {
                    self.plots.push(PlotItem::default_polar());
                    ui.close_menu();
                }

                if ui.button("Equation").clicked() {
                    //TODO
                    ui.close_menu();
                }

                if ui.button("High plot").clicked() {
                    //TODO
                    ui.close_menu();
                }
            });

            ui.menu_button("Data 2d", |ui| {
                if ui.button("Scatter").clicked() {
                    //TODO
                    ui.close_menu();
                }

                if ui.button("Linear").clicked() {
                    //TODO
                    ui.close_menu();
                }

                if ui.button("Histogram").clicked() {
                    //TODO
                    ui.close_menu();
                }
            });

            ui.menu_button("Plot 3d", |ui| {
                if ui.button("Parametric").clicked() {
                    //TODO
                    ui.close_menu();
                }

                if ui.button("Equation").clicked() {
                    //TODO
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
