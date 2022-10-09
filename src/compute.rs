use meval::Expr;
use crate::PlotType;
use crate::app::Plotter;

impl Plotter {
    pub fn compute_all(&mut self) {
        for i in 0..self.plots.len() {
            match self.plots[i].plot_type {
                PlotType::Function2d => {
                    self.compute_function_equal_grid(i);
                }
                PlotType::Parametric2d => {
                    self.compute_parametric_equal_grid(i);
                }
                PlotType::Polar2d => {
                    self.compute_polar_equal_grid(i)
                }
                _ => {} // TODO
            }
        }
    }

    pub fn parse_equal_grid(&mut self, plot_number: usize) {
        let n_r = meval::eval_str(self.plots[plot_number].n_s.to_owned());
        let a_r = meval::eval_str(self.plots[plot_number].a_s.to_owned());
        let b_r = meval::eval_str(self.plots[plot_number].b_s.to_owned());

        let mut has_an_error = false;

        match n_r {
            Ok(n) => {
                self.plots[plot_number].n = n as u32;
            }
            Err(e) => {
                self.plots[plot_number].error_message = e.to_string();
                has_an_error = true;
            }
        };

        match a_r {
            Ok(a) => {
                self.plots[plot_number].a = a;
            }
            Err(e) => {
                self.plots[plot_number].error_message = e.to_string();
                has_an_error = true;
            }
        };

        match b_r {
            Ok(b) => {
                self.plots[plot_number].b = b;
            }
            Err(e) => {
                self.plots[plot_number].error_message = e.to_string();
                has_an_error = true;
            }
        };

        self.plots[plot_number].has_an_error = has_an_error;
    }

    pub fn compute_function_equal_grid(&mut self, plot_number: usize) {
        if self.plots[plot_number].are_data_computed {
            return;
        }

        self.parse_equal_grid(plot_number);
        if self.plots[plot_number].has_an_error {
            return;
        }
        let f = self.get_parsed_function(plot_number, self.plots[plot_number].function.to_owned(), "x".to_owned());
        if self.plots[plot_number].has_an_error {
            return;
        }

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

    pub fn compute_parametric_equal_grid(&mut self, plot_number: usize) {
        if self.plots[plot_number].are_data_computed {
            return;
        }

        self.parse_equal_grid(plot_number);
        if self.plots[plot_number].has_an_error {
            return;
        }
        let x = self.get_parsed_function(plot_number, self.plots[plot_number].function.to_owned(), "t".to_owned());
        if self.plots[plot_number].has_an_error {
            return;
        }
        let y = self.get_parsed_function(plot_number, self.plots[plot_number].parameter.to_owned(), "t".to_owned());
        if self.plots[plot_number].has_an_error {
            return;
        }

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

    pub fn compute_polar_equal_grid(&mut self, plot_number: usize) {
        if self.plots[plot_number].are_data_computed {
            return;
        }

        self.parse_equal_grid(plot_number);
        if self.plots[plot_number].has_an_error {
            return;
        }
        let r = self.get_parsed_function(plot_number, self.plots[plot_number].function.to_owned(), "a".to_owned());
        if self.plots[plot_number].has_an_error {
            return;
        }

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

    fn get_parsed_function(&mut self, plot_number: usize, string: String, variable: String) -> impl Fn(f64) -> f64 {
        let mut has_an_error = false;

        let expr = match string.parse::<Expr>() {
            Ok(res) => res,
            Err(e) => {
                self.plots[plot_number].error_message = e.to_string();
                has_an_error = true;
                variable.parse().unwrap()
            }
        };

        let function = match expr.bind(variable.as_str()) {
            Ok(res) => res,
            Err(e) => {
                self.plots[plot_number].error_message = e.to_string();
                has_an_error = true;
                variable.parse::<Expr>().unwrap().bind(variable.as_str()).unwrap()
            }
        };

        self.plots[plot_number].has_an_error = has_an_error;

        return function;
    }
}
