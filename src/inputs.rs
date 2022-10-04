use egui::{Color32, RichText};
use crate::PlotType;

impl crate::Plotter {
    //////// Input forms for different plot types ////////

    pub fn input(&mut self, ui: &mut egui::Ui) {
        match self.plot_type {

            // 2D
            PlotType::Function2d => {
                self.input_function2d(ui);
            }
            PlotType::Parametric2d => {
                self.input_parametric2d(ui);
            }
            PlotType::Polar2d => {
                self.input_polar2d(ui);
            }
            PlotType::Equation2d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
            PlotType::High2d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }

            // Data
            PlotType::Scatter2d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
            PlotType::Linear2d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
            PlotType::Histogram2d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }

            // 3D

            PlotType::Parametric3d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
            PlotType::Equation3d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
        }
    }

    fn input_function2d(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("y(x): ");
            if ui.text_edit_singleline(&mut self.function).changed() {
                self.are_data_computed = false;
            }
        });

        self.input_uniform_grid(ui);
    }

    fn input_parametric2d(&mut self, ui: &mut egui::Ui) {

        ui.horizontal(|ui| {
            ui.label("x(t): ");
            if ui.text_edit_singleline(&mut self.parametric1).changed() {
                self.are_data_computed = false;
            }
        });

        ui.horizontal(|ui| {
            ui.label("y(t): ");
            if ui.text_edit_singleline(&mut self.parametric2).changed() {
                self.are_data_computed = false;
            }
        });

        self.input_uniform_grid(ui);
    }

    fn input_polar2d(&mut self, ui: &mut egui::Ui) {
        self.polar = self.input_filed_string(ui, "r(a): ".to_string(), self.polar.to_owned());

        self.input_uniform_grid(ui);
    }

    //////// Common patterns ////////

    fn input_uniform_grid(&mut self, ui: &mut egui::Ui) {
        self.lower_limit = self.input_filed_string(ui, "a: ".to_string(), self.lower_limit.to_owned());
        self.higher_limit = self.input_filed_string(ui, "b: ".to_string(), self.higher_limit.to_owned());
        self.intervals_amount = self.input_filed_string(ui, "n: ".to_string(), self.intervals_amount.to_owned());
    }

    fn input_filed_string(&mut self, ui: &mut egui::Ui, label: String, initial: String) -> String {
        let mut result: String = initial;

        ui.horizontal(|ui| {
            ui.label(label);
            if ui.text_edit_singleline(&mut result).changed() {
                self.are_data_computed = false;
            };
        });

        return result;
    }
}
