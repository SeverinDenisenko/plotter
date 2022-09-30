
impl crate::Plotter {
    pub fn parse_equal_grid(&mut self) -> (f64, f64, u32){
        let a_t = self.lower_limit.parse();
        let b_t = self.higher_limit.parse();
        let n_t = self.intervals_amount.parse();

        let a: f64 = match a_t {
            Ok(result) => result,
            Err(_) => 0.0
        };

        let b: f64 = match b_t {
            Ok(result) => result,
            Err(_) => 0.0
        };

        let n: u32 = match n_t {
            Ok(result) => result,
            Err(_) => 0
        };

        return (a, b, n);
    }

    pub fn compute_equal_grid(&mut self, a: f64, b: f64, n: u32){
        if !self.are_data_computed {
            self.points.clear();

            for i in 0..n + 1 {
                let x = a + i as f64 * (b - a) / n as f64;

                //TODO function parsing
                self.points.push([x, x.sin()]);
            }

            self.are_data_computed = true;
        }
    }

    pub fn compute_parametric_equal_grid(&mut self, a: f64, b: f64, n: u32){
        if !self.are_data_computed {

            self.points.clear();
            self.parameter.clear();

            for i in 0..n + 1 {
                let t = a + i as f64 * (b - a) / n as f64;
                self.parameter.push(t);

                //TODO function parsing
                self.points.push([t.cos(), t.sin()]);
            }

            self.are_data_computed = true;
        }
    }

}