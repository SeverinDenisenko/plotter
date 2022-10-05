pub enum PlotType {
    Function2d,
    Parametric2d,
    Polar2d,
    Equation2d,
    High2d,
    Scatter2d,
    Linear2d,
    Histogram2d,
    Parametric3d,
    Equation3d,
}

pub enum PlotProvider {
    Egui,
    Plotters,
    Poloto,
}

pub struct PlotItem {
    pub input_function: String,
    pub input_parameter: String,
    // May be empty for non-parametric inputs
    pub lower_limit_s: String,
    pub higher_limit_s: String,
    pub intervals_amount_s: String,

    pub plot_type: PlotType,
    pub lower_limit: f64,
    pub higher_limit: f64,
    pub intervals_amount: u32,
    pub points: Vec<[f64; 2]>,
    pub are_data_computed: bool,
}

impl PlotItem {
    pub fn default_function() -> PlotItem {
        PlotItem {
            input_function: "exp(x)".to_owned(),
            input_parameter: "".to_owned(), // May be empty for non-parametric inputs
            lower_limit_s: "-1".to_owned(),
            higher_limit_s: "1".to_owned(),
            intervals_amount_s: "100".to_owned(),

            plot_type: PlotType::Function2d,
            lower_limit: -1.0,
            higher_limit: 1.0,
            intervals_amount: 100,
            points: vec![],
            are_data_computed: false,
        }
    }

    pub fn default_polar() -> PlotItem {
        PlotItem {
            input_function: "cos(a * 3)".to_owned(),
            input_parameter: "".to_owned(), // May be empty for non-parametric inputs
            lower_limit_s: "0".to_owned(),
            higher_limit_s: "pi * 2".to_owned(),
            intervals_amount_s: "100".to_owned(),

            plot_type: PlotType::Polar2d,
            lower_limit: 0.0,
            higher_limit: std::f64::consts::PI * 2.0,
            intervals_amount: 100,
            points: vec![],
            are_data_computed: false,
        }
    }

    pub fn default_parametric() -> PlotItem {
        PlotItem {
            input_function: "sin(t)".to_owned(),
            input_parameter: "cos(t)".to_owned(), // May be empty for non-parametric inputs
            lower_limit_s: "0".to_owned(),
            higher_limit_s: "pi * 2".to_owned(),
            intervals_amount_s: "100".to_owned(),

            plot_type: PlotType::Parametric2d,
            lower_limit: 0.0,
            higher_limit: std::f64::consts::PI * 2.0,
            intervals_amount: 100,
            points: vec![],
            are_data_computed: false,
        }
    }
}
