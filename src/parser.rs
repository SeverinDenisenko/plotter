use meval::{Expr};

pub fn parse_function(string: String, variable: String) -> Result<impl Fn(f64) -> f64, String> {
    let expr = match string.parse::<Expr>() {
        Ok(res) => res,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let function = match expr.bind(variable.as_str()) {
        Ok(res) => res,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    return Ok(function);
}

pub fn parse_float(string: String) -> Result<f64, String>{
    match meval::eval_str(string) {
        Ok(f) => Ok(f),
        Err(e) => Err(e.to_string())
    }
}

pub fn parse_uint(string: String) -> Result<u32, String>{
    match meval::eval_str(string) {
        Ok(f) => Ok(f as u32),
        Err(e) => Err(e.to_string())
    }
}

