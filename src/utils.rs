use meval::Expr;

impl crate::Plotter {
    pub fn parse_equal_grid(&mut self) -> (f64, f64, u32) {
        let n_t = self.intervals_amount.parse();

        let a = Self::evaluate_string(self.lower_limit.to_owned());
        let b = Self::evaluate_string(self.higher_limit.to_owned());

        let n: u32 = match n_t {
            Ok(result) => result,
            Err(_) => 0
        };

        return (a, b, n);
    }

    pub fn compute_equal_grid(&mut self, a: f64, b: f64, n: u32) {
        if !self.are_data_computed {
            let f = Self::get_parsed_function(self.function.to_owned(), "x".to_owned());

            self.points.clear();

            for i in 0..n + 1 {
                let x = a + i as f64 * (b - a) / n as f64;

                self.points.push([x, f(x)]);
            }

            self.are_data_computed = true;
        }
    }

    pub fn compute_parametric_equal_grid(&mut self, a: f64, b: f64, n: u32) {
        if !self.are_data_computed {
            let x = Self::get_parsed_function(self.parametric1.to_owned(), "t".to_owned());
            let y = Self::get_parsed_function(self.parametric2.to_owned(), "t".to_owned());

            self.points.clear();

            for i in 0..n + 1 {
                let t = a + i as f64 * (b - a) / n as f64;

                self.points.push([x(t), y(t)]);
            }

            self.are_data_computed = true;
        }
    }

    pub fn compute_polar_equal_grid(&mut self, a: f64, b: f64, n: u32) {
        if !self.are_data_computed {
            let r = Self::get_parsed_function(self.polar.to_owned(), "a".to_owned());

            self.points.clear();

            for i in 0..n + 1 {
                let t = a + i as f64 * (b - a) / n as f64;
                let z = r(t);

                self.points.push([z * t.cos(), z * t.sin()]);
            }

            self.are_data_computed = true;
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