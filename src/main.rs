#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::env;

mod app;
mod plot_type;
mod compute;
mod plot;
mod inputs;
mod parser;
mod runner;

use plot_type::*;
use runner::run_default;

fn main() {
    let args: Vec<String> = env::args().collect();

    // If run without command line args display plotting window
    if args.len() == 1 {
        run_default();
        return;
    }


}
