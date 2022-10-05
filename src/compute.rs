use meval::Expr;
use crate::PlotType;

impl crate::Plotter {
    pub fn compute_all(&mut self){
        for i in 0..self.plots.len() {
            match self.plots[i].plot_type {
                PlotType::Function2d => {
                    self.compute_function_equal_grid(i);
                },
                PlotType::Parametric2d => {
                    self.compute_parametric_equal_grid(i);
                },
                PlotType::Polar2d => {
                    self.compute_polar_equal_grid(i)
                },
                _ => {} // TODO
            }
        }
    }

    pub fn parse_equal_grid(&mut self, plot_number: usize) {
        self.plots[plot_number].n = Self::evaluate_string(self.plots[plot_number].n_s.to_owned()) as u32;

        self.plots[plot_number].a = Self::evaluate_string(self.plots[plot_number].a_s.to_owned());
        self.plots[plot_number].b = Self::evaluate_string(self.plots[plot_number].b_s.to_owned());
    }

    pub fn compute_function_equal_grid(&mut self, plot_number: usize) {
        if !self.plots[plot_number].are_data_computed {
            self.parse_equal_grid(plot_number);
            let f = Self::get_parsed_function(self.plots[plot_number].function.to_owned(), "x".to_owned());

            self.plots[plot_number].points.clear();

            let n = self.plots[plot_number].n;
            let a = self.plots[plot_number].a;
            let b = self.plots[plot_number].b;

            for i in 0..n + 1 {
                let x = a + i as f64 * (b - a) / n as f64;

                self.plots[plot_number].points.push([x, f(x)]);
            }

            self.plots[plot_number].are_data_computed = true;
        }
    }

    pub fn compute_parametric_equal_grid(&mut self, plot_number: usize) {
        if !self.plots[plot_number].are_data_computed {
            self.parse_equal_grid(plot_number);

            let x = Self::get_parsed_function(self.plots[plot_number].function.to_owned(), "t".to_owned());
            let y = Self::get_parsed_function(self.plots[plot_number].parameter.to_owned(), "t".to_owned());

            self.plots[plot_number].points.clear();

            let n = self.plots[plot_number].n;
            let a = self.plots[plot_number].a;
            let b = self.plots[plot_number].b;

            for i in 0..n + 1 {
                let t = a + i as f64 * (b - a) / n as f64;

                self.plots[plot_number].points.push([x(t), y(t)]);
            }

            self.plots[plot_number].are_data_computed = true;
        }
    }

    pub fn compute_polar_equal_grid(&mut self, plot_number: usize) {
        if !self.plots[plot_number].are_data_computed {
            self.parse_equal_grid(plot_number);

            let r = Self::get_parsed_function(self.plots[plot_number].function.to_owned(), "a".to_owned());

            self.plots[plot_number].points.clear();

            let n = self.plots[plot_number].n;
            let a = self.plots[plot_number].a;
            let b = self.plots[plot_number].b;

            for i in 0..n + 1 {
                let t = a + i as f64 * (b - a) / n as f64;
                let z = r(t);

                self.plots[plot_number].points.push([z * t.cos(), z * t.sin()]);
            }

            self.plots[plot_number].are_data_computed = true;
        }
    }

    fn get_parsed_function(string: String, variable: String) -> impl Fn(f64) -> f64 {
        let expr = match string.parse::<Expr>() {
            Ok(res) => res,
            Err(_) => variable.parse().unwrap()
        };

        let function = match expr.bind(variable.as_str()) {
            Ok(res) => res,
            Err(_) => variable.parse::<Expr>().unwrap().bind(variable.as_str()).unwrap()
        };

        return function;
    }

    fn evaluate_string(string: String) -> f64 {
        let val: f64 = match meval::eval_str(string) {
            Ok(result) => result,
            Err(_) => 0.0
        };

        return val;
    }
}