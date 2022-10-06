use egui::{Color32, RichText, ScrollArea, TextStyle};
use crate::types::*;

impl crate::Plotter {
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
                        },
                        PlotType::Parametric2d => {
                            self.input_parametric2d(ui, i);
                        },
                        PlotType::Polar2d => {
                            self.input_polar2d(ui, i);
                        }
                        _ => {}, // TODO
                    }

                    ui.add_space(3.0);

                    if self.plots[i].has_an_error {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Error!").color(Color32::RED));
                        });
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Remove").clicked() && self.plots.len() != 1 {
                            plots_to_remove.push(i);
                        }

                        egui::Ui::color_edit_button_rgb(ui, &mut self.plots[i].color);
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
