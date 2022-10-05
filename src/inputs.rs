use egui::ScrollArea;
use crate::types::*;

impl crate::Plotter {
    //////// Input forms for different plot types ////////

    pub fn input_all(&mut self, ui: &mut egui::Ui) {

        let mut plots_to_remove: Vec<usize> = vec![];

        ui.add_space(4.0);

        let num_rows = self.plots.len();
        let row_height = 50.0;

        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show_rows(
            ui,
            row_height,
            num_rows,
            |ui, _| {

                for i in 0..self.plots.len() {
                    match self.plots[i].plot_type {

                        // 2D
                        PlotType::Function2d => {
                            self.input_function2d(ui, i);
                        },
                        PlotType::Parametric2d => {
                            self.input_parametric2d(ui, i);
                        },
                        PlotType::Polar2d => {
                            self.input_polar2d(ui, i);
                        }
                        _ => {}, // TODO
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Remove").clicked() && self.plots.len() != 1 {
                            plots_to_remove.push(i);
                        }
                    });

                    ui.separator();
                }
            },
        );

        for i in plots_to_remove {
            self.plots.remove(i);
        }
    }

    fn input_function2d(&mut self, ui: &mut egui::Ui, plot_number: usize) {
        ui.horizontal(|ui| {
            ui.label("y(x): ");
            if ui.text_edit_singleline(&mut self.plots[plot_number].function).changed() {
                self.plots[plot_number].are_data_computed = false;
            }
        });

        self.input_uniform_grid(ui, plot_number);
    }

    fn input_parametric2d(&mut self, ui: &mut egui::Ui, plot_number: usize) {
        ui.horizontal(|ui| {
            ui.label("x(t): ");
            if ui.text_edit_singleline(&mut self.plots[plot_number].function).changed() {
                self.plots[plot_number].are_data_computed = false;
            }
        });

        ui.horizontal(|ui| {
            ui.label("y(t): ");
            if ui.text_edit_singleline(&mut self.plots[plot_number].parameter).changed() {
                self.plots[plot_number].are_data_computed = false;
            }
        });

        self.input_uniform_grid(ui, plot_number);
    }

    fn input_polar2d(&mut self, ui: &mut egui::Ui, plot_number: usize) {
        self.plots[plot_number].function = self.input_filed_string(ui, "r(a): ".to_string(), self.plots[plot_number].function.to_owned(), plot_number);

        self.input_uniform_grid(ui, plot_number);
    }

    //////// Common patterns ////////

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
