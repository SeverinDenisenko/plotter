use rand;

#[derive(PartialEq)]
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
    pub function: String,
    pub parameter: String,
    // May be empty for non-parametric inputs
    pub a_s: String,
    pub b_s: String,
    pub n_s: String,

    pub plot_type: PlotType,
    pub a: f64,
    pub b: f64,
    pub n: u32,
    pub points: Vec<[f64; 2]>,
    pub are_data_computed: bool,

    pub name: String,

    pub has_an_error: bool,
    pub error_message: String,

    pub color: [f32; 3],
}

impl PlotItem {
    pub fn default_function() -> PlotItem {
        PlotItem {
            function: "exp(x)".to_owned(),
            parameter: "".to_owned(), // Empty for non-parametric inputs
            a_s: "-1".to_owned(),
            b_s: "1".to_owned(),
            n_s: "100".to_owned(),

            plot_type: PlotType::Function2d,
            a: -1.0,
            b: 1.0,
            n: 100,
            points: vec![],
            are_data_computed: false,

            // Only for data's plot type
            name: "".to_owned(),

            has_an_error: false,
            error_message: "".to_owned(),

            color: [rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()]
        }
    }

    pub fn default_polar() -> PlotItem {
        PlotItem {
            function: "cos(a * 3)".to_owned(),
            parameter: "".to_owned(), // May be empty for non-parametric inputs
            a_s: "0".to_owned(),
            b_s: "pi * 2".to_owned(),
            n_s: "100".to_owned(),

            plot_type: PlotType::Polar2d,
            a: 0.0,
            b: std::f64::consts::PI * 2.0,
            n: 100,
            points: vec![],
            are_data_computed: false,

            // Only for data's plot type
            name: "".to_owned(),

            has_an_error: false,
            error_message: "".to_owned(),

            color: [rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()]
        }
    }

    pub fn default_parametric() -> PlotItem {
        PlotItem {
            function: "sin(t)".to_owned(),
            parameter: "cos(t)".to_owned(), // May be empty for non-parametric inputs
            a_s: "0".to_owned(),
            b_s: "pi * 2".to_owned(),
            n_s: "100".to_owned(),

            plot_type: PlotType::Parametric2d,
            a: 0.0,
            b: std::f64::consts::PI * 2.0,
            n: 100,
            points: vec![],
            are_data_computed: false,

            // Only for data's plot type
            name: "".to_owned(),

            has_an_error: false,
            error_message: "".to_owned(),

            color: [rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()]
        }
    }

    pub fn default_linear_2d() -> PlotItem {
        PlotItem {
            function: "".to_owned(),
            parameter: "".to_owned(), // May be empty for non-parametric inputs
            a_s: "".to_owned(),
            b_s: "".to_owned(),
            n_s: "".to_owned(),

            plot_type: PlotType::Linear2d,
            a: 0.0,
            b: 0.0,
            n: 0,
            points: vec![],
            are_data_computed: false,

            // Only for data's plot type
            name: "".to_owned(),

            has_an_error: false,
            error_message: "".to_owned(),

            color: [rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()]
        }
    }
}
