use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use egui::{Color32, RichText, ScrollArea, TextStyle};
use rfd::FileDialog;

use crate::plot_type::*;
use crate::app::Plotter;


impl Plotter {
    //////// Input forms for different plot types ////////

    pub fn input_all(&mut self, ui: &mut egui::Ui) {
        let mut plots_to_remove: Vec<usize> = vec![];

        ui.add_space(4.0);

        let num_rows = self.plots.len();

        let text_style = TextStyle::Body; // This does not make any scene
        let row_height = ui.text_style_height(&text_style) * 7.8;

        ScrollArea::vertical()
            .auto_shrink([false; 2]).stick_to_bottom(false)
            .show_rows(
                ui,
                row_height,
                num_rows,
                |ui, row| {
                    for i in row {
                        match self.plots[i].plot_type {
                            // 2D
                            PlotType::Function2d => {
                                self.input_function2d(ui, i);
                            }
                            PlotType::Parametric2d => {
                                self.input_parametric2d(ui, i);
                            }
                            PlotType::Polar2d => {
                                self.input_polar2d(ui, i);
                            }
                            // Data
                            PlotType::PointsXY2d => {
                                self.input_linear_2d(ui, i);
                            },
                            PlotType::PointsY2d => {
                                self.input_linear_2d(ui, i);
                            }
                            _ => {} // TODO
                        }

                        ui.add_space(3.0);

                        if self.plots[i].has_an_error {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(self.plots[i].error_message.to_owned()).color(Color32::RED));
                            });
                        }

                        ui.horizontal(|ui| {
                            if ui.button("Remove").clicked() {
                                plots_to_remove.push(i);
                            }

                            egui::Ui::color_edit_button_rgb(ui, &mut self.plots[i].color);

                            let cmb_text = match self.plots[i].plot_style {
                                PlotStyle::Lines => "Lines",
                                PlotStyle::Points => "Points"
                            };

                            egui::ComboBox::from_id_source(i).selected_text(cmb_text)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.plots[i].plot_style, PlotStyle::Lines, "Lines");
                                    ui.selectable_value(&mut self.plots[i].plot_style, PlotStyle::Points, "Points");
                                }
                                );
                        });

                        ui.separator();
                    }
                },
            );

        ui.add_space(4.0);

        for i in plots_to_remove {
            self.plots.remove(i);
        }
    }

    // Input 2d

    fn input_function2d(&mut self, ui: &mut egui::Ui, plot_number: usize) {
        self.plots[plot_number].function = self.input_filed_string(ui, "y(x): ".to_string(), self.plots[plot_number].function.to_owned(), plot_number);

        self.input_uniform_grid(ui, plot_number);
    }

    fn input_parametric2d(&mut self, ui: &mut egui::Ui, plot_number: usize) {
        self.plots[plot_number].function = self.input_filed_string(ui, "x(t): ".to_string(), self.plots[plot_number].function.to_owned(), plot_number);
        self.plots[plot_number].parameter = self.input_filed_string(ui, "y(t): ".to_string(), self.plots[plot_number].parameter.to_owned(), plot_number);

        self.input_uniform_grid(ui, plot_number);
    }

    fn input_polar2d(&mut self, ui: &mut egui::Ui, plot_number: usize) {
        self.plots[plot_number].function = self.input_filed_string(ui, "r(a): ".to_string(), self.plots[plot_number].function.to_owned(), plot_number);

        self.input_uniform_grid(ui, plot_number);
    }

    // Input data

    fn input_linear_2d(&mut self, ui: &mut egui::Ui, plot_number: usize) {
        if ui.button("Open").clicked() {
            match self.plots[plot_number].plot_type {
                PlotType::PointsXY2d => {
                    self.get_data_x_y(plot_number);
                },
                PlotType::PointsY2d => {
                    self.get_data_y(plot_number);
                },
                _ => {
                    //TODO
                }
            }

        }

        ui.label(RichText::new(self.plots[plot_number].name.to_owned()).color(Color32::BROWN));
    }

    //////// Common patterns ////////

    pub fn load_data_x_y(&mut self, path: &Path, plot_number: usize){
        let file = match File::open(path) {
            Ok(f) => f,
            Err(err) => {
                self.plots[plot_number].has_an_error = true;
                self.plots[plot_number].error_message = err.to_string();
                return;
            }
        };

        let lines: Vec<String> = BufReader::new(file).lines().map(|l| {
            match l {
                Ok(str) => str,
                Err(err) => {
                    self.plots[plot_number].has_an_error = true;
                    self.plots[plot_number].error_message = err.to_string();
                    "".to_owned()
                }
            }
        }).collect();

        self.plots[plot_number].name = path.file_name().unwrap().to_str().unwrap().to_string();

        self.plots[plot_number].n = 0;
        self.plots[plot_number].points.clear();

        for line in lines {
            let nums: Vec<&str> = line.split(" ").collect();
            if nums.len() != 2 {
                continue;
            }

            let a_s = nums[0];
            let b_s = nums[1];

            let a = match meval::eval_str(a_s) {
                Ok(num) => num,
                Err(err) => {
                    self.plots[plot_number].has_an_error = true;
                    self.plots[plot_number].error_message = err.to_string();
                    continue;
                }
            };

            let b = match meval::eval_str(b_s) {
                Ok(num) => num,
                Err(err) => {
                    self.plots[plot_number].has_an_error = true;
                    self.plots[plot_number].error_message = err.to_string();
                    continue;
                }
            };

            self.plots[plot_number].points.push([a, b]);
            self.plots[plot_number].n += 1;
        }

        self.plots[plot_number].are_data_computed = true;
    }

    pub fn load_data_y(&mut self, path: &Path, plot_number: usize){
        let file = match File::open(path) {
            Ok(f) => f,
            Err(err) => {
                self.plots[plot_number].has_an_error = true;
                self.plots[plot_number].error_message = err.to_string();
                return;
            }
        };

        let lines: Vec<String> = BufReader::new(file).lines().map(|l| {
            match l {
                Ok(str) => str,
                Err(err) => {
                    self.plots[plot_number].has_an_error = true;
                    self.plots[plot_number].error_message = err.to_string();
                    "".to_owned()
                }
            }
        }).collect();

        self.plots[plot_number].name = path.file_name().unwrap().to_str().unwrap().to_string();

        self.plots[plot_number].n = 0;
        self.plots[plot_number].points.clear();

        for line in lines {

            let y_s = line;

            let y = match meval::eval_str(y_s) {
                Ok(num) => num,
                Err(err) => {
                    self.plots[plot_number].has_an_error = true;
                    self.plots[plot_number].error_message = err.to_string();
                    continue;
                }
            };

            let x = self.plots[plot_number].n as f64;
            self.plots[plot_number].points.push([x, y]);
            self.plots[plot_number].n += 1;
        }

        self.plots[plot_number].are_data_computed = true;
    }

    fn get_data_x_y(&mut self, plot_number: usize) {
        let path_o = FileDialog::new().pick_file();

        let path = match path_o {
            Some(f) => f,
            None => {
                return;
            }
        };

        self.load_data_x_y(path.as_ref(), plot_number);
    }

    fn get_data_y(&mut self, plot_number: usize) {
        let path_o = FileDialog::new().pick_file();

        let path = match path_o {
            Some(f) => f,
            None => {
                return;
            }
        };

        self.load_data_y(path.as_ref(), plot_number);
    }

    fn input_uniform_grid(&mut self, ui: &mut egui::Ui, plot_number: usize) {
        self.plots[plot_number].a_s = self.input_filed_string(ui, "a: ".to_string(), self.plots[plot_number].a_s.to_owned(), plot_number);
        self.plots[plot_number].b_s = self.input_filed_string(ui, "b: ".to_string(), self.plots[plot_number].b_s.to_owned(), plot_number);
        self.plots[plot_number].n_s = self.input_filed_string(ui, "n: ".to_string(), self.plots[plot_number].n_s.to_owned(), plot_number);
    }

    fn input_filed_string(&mut self, ui: &mut egui::Ui, label: String, initial: String, plot_number: usize) -> String {
        let mut result: String = initial;

        ui.horizontal(|ui| {
            ui.label(label);
            if ui.text_edit_singleline(&mut result).changed() {
                self.plots[plot_number].are_data_computed = false;
            };
        });

        return result;
    }
}
