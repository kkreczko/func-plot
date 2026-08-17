#[derive(Debug)]
pub enum RangeParseError {
    NotCorrectRangeString,
}

pub fn generate_range(mut min: f64, max: f64, step: f64) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();

    while min <= max {
        result.push(min);
        min += step;
    }

    result
}

pub fn parse_range(range: &str) -> Result<(f64, f64, f64), RangeParseError> {
    let mut result: Vec<f64> = Vec::new();
    let splitted: Vec<&str> = range.split(";").collect();

    for word in splitted {
        match word.parse::<f64>() {
            Ok(number) => result.push(number),
            Err(_) => {
                return Err(RangeParseError::NotCorrectRangeString);
            }
        }
    }

    return Ok((result[0], result[1], result[2]));
}
