#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::env;
use std::path::Path;

mod app;
mod plot_type;
mod compute;
mod plot;
mod inputs;
mod parser;
mod runner;

use plot_type::*;
use runner::run_default;
use crate::app::Plotter;
use crate::runner::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    // If run without command line args display plotting window
    if args.len() == 1 {
        run_default();
        return;
    }

    // Parse config file
    if args.len() == 2 {
        //TODO
        return;
    }

    if args.len() != 3 {
        panic!("Wrong number of arguments.");
    }

    let plot_type = args.get(1).unwrap().to_string();

    let plot_type_function: String = String::from("function");
    let plot_type_data: String = String::from("data");

    if plot_type == plot_type_function {
        let mut plotter = Plotter::default();
        plotter.plots.push(PlotItem::default_function());

        plotter.plots[0].function = args.get(2).unwrap().to_string();

        run(plotter);
        return;
    }

    if plot_type == plot_type_data {
        let mut plotter = Plotter::default();
        plotter.plots.push(PlotItem::default_points_x_y_2d());

        let path_str = args.get(2).unwrap().to_string();
        let path = Path::new(&path_str);

        plotter.load_data_x_y(path, 0);

        run(plotter);
        return;
    }
}
