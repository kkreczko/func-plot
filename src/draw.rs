use ::std::num;

// calculate x or y bounds
fn calculate_bounds(values: &[f64]) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    for value in values {
        if value.is_finite() {
            min = min.min(*value);
            max = max.max(*value);
        }
    }

    if min.is_infinite() {
        return None;
    }

    Some((min, max))
}

pub fn get_dimension(values: &[f64]) -> i32 {
    match calculate_bounds(values) {
        Some((min, max)) => (min.abs() + max.abs()) as i32,
        None => -1,
    }
}

pub fn coordinates_to_pixels(x: f64, y: f64, screen_width: i32, screen_height: i32) -> (i32, i32) {
    (1, 0)
}

// TODO draw x and y axis
// TODO get x_min and x_max from range min and x_max
// TODO translate x and y values to screen coordinates
// TODO draw ticks on axes
// TODO draw lines between two points by translating x and y to screen cords
// This flow only gets range min, max, arguments and values and raylib context then draws to full resolution on raylib
// canvas
