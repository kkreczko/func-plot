use::std::num;

pub fn calculate_y_axis_bounds(values: &[f64]) -> Option<(f64, f64)> {
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

// TODO draw x and y axis
// TODO get x_min and x_max from range min and x_max
// TODO translate x and y values to screen coordinates
// TODO draw ticks on axes
// TODO draw lines between two points by translating x and y to screen cords
// This flow only gets range min, max, arguments and values then draws to full resolution on raylib
// canvas
