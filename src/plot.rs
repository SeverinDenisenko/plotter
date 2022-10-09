use egui::plot::{Line, Plot, PlotPoints, Points};
use egui::{Color32, Rgba};

use crate::plot_type::*;
use crate::app::Plotter;

impl Plotter {

    //////// Plots for different plot types and providers ////////

    pub fn plot_all(&mut self, ui: &mut egui::Ui) {
        match self.plot_provider {
            PlotProvider::Egui => {
                self.plot_all_egui(ui);
            },
            PlotProvider::Plotters => {
                // TODO
            },
            PlotProvider::Poloto => {
                // TODO
            }
        }

    }

    //////// Specific plot functions for providers ////////

    fn plot_all_egui(&mut self, ui: &mut egui::Ui){
        self.compute_all();

        let mut lines: Vec<Line> = vec![];
        let mut points: Vec<Points> = vec![];

        for j in 0..self.plots.len() {
            if self.plots[j].plot_style == PlotStyle::Lines {
                lines.push(
                    Line::new(
                        (0..self.plots[j].points.len()).map(
                            |i| {
                                self.plots[j].points[i as usize]
                            }
                        ).collect::<PlotPoints>()
                    ).color(Color32::from(Rgba::from_rgb(self.plots[j].color[0],
                                                         self.plots[j].color[1],
                                                         self.plots[j].color[2])))
                )
            }
            if self.plots[j].plot_style == PlotStyle::Points {
                points.push(
                    Points::new((0..self.plots[j].points.len()).map(
                        |i| {
                            self.plots[j].points[i as usize]
                        }
                    ).collect::<PlotPoints>()).color(Color32::from(Rgba::from_rgb(self.plots[j].color[0],
                                                                                  self.plots[j].color[1],
                                                                                  self.plots[j].color[2])))
                )
            }
        }

        Plot::new("Plot")
            .data_aspect(1.0)
            .show(ui, |plot_ui| {
                for line in lines {
                    plot_ui.line(line)
                }
                for point in points {
                    plot_ui.points(point)
                }
            });
    }
}
