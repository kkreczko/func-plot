#[derive(Debug, PartialEq)]
pub enum RangeParseError {
    IncorrectRangeString,
}

pub fn generate_range(min: f64, max: f64, step: f64) -> Vec<f64> {
    if !min.is_finite() || !max.is_finite() || !step.is_finite() || min >= max || step <= 0.0 {
        return Vec::new();
    }
    let mut result = Vec::new();
    let count = ((max - min) / step).floor().min(200_000.0) as usize;
    for index in 0..=count {
        let x = min + index as f64 * step;
        if x <= max {
            result.push(x);
        }
    }
    if result.last().is_some_and(|last| *last < max) && result.len() < 200_001 {
        result.push(max);
    }
    result
}

pub fn parse_range(range: &str) -> Result<(f64, f64, f64), RangeParseError> {
    let parts: Vec<&str> = range.split(';').collect();
    if parts.len() != 3 {
        return Err(RangeParseError::IncorrectRangeString);
    }
    let numbers: Vec<f64> = parts
        .iter()
        .map(|part| part.trim().parse::<f64>())
        .collect::<Result<_, _>>()
        .map_err(|_| RangeParseError::IncorrectRangeString)?;
    let (min, max, step) = (numbers[0], numbers[1], numbers[2]);
    if !min.is_finite() || !max.is_finite() || !step.is_finite() || min >= max || step <= 0.0 {
        return Err(RangeParseError::IncorrectRangeString);
    }
    Ok((min, max, step))
}
