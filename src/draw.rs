/// Geometry for a plot with fixed pixel margins around the graph.
#[derive(Debug, Clone, Copy)]
pub struct PlotArea {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

impl PlotArea {
    pub fn new(
        screen_width: i32,
        screen_height: i32,
        top: i32,
        x_min: f64,
        x_max: f64,
        curves: &[Vec<(f64, f64)>],
    ) -> Option<Self> {
        if !x_min.is_finite() || !x_max.is_finite() || x_min >= x_max {
            return None;
        }
        let (mut y_min, mut y_max) = (f64::INFINITY, f64::NEG_INFINITY);
        for (_, y) in curves.iter().flat_map(|curve| curve.iter()) {
            if y.is_finite() {
                y_min = y_min.min(*y);
                y_max = y_max.max(*y);
            }
        }
        if !y_min.is_finite() || !y_max.is_finite() {
            return None;
        }
        let span = y_max - y_min;
        let padding = if span > 0.0 && span.is_finite() {
            span * 0.08
        } else {
            y_min.abs().max(1.0) * 0.1
        };
        y_min -= padding;
        y_max += padding;
        Some(Self {
            x_min,
            x_max,
            y_min,
            y_max,
            left: 72,
            top,
            width: screen_width - 72 - 24,
            height: screen_height - top - 58,
        })
    }

    pub fn to_pixels(&self, x: f64, y: f64) -> Option<(f32, f32)> {
        if !x.is_finite() || !y.is_finite() {
            return None;
        }
        Some((
            (self.left as f64 + (x - self.x_min) / (self.x_max - self.x_min) * self.width as f64)
                as f32,
            (self.top as f64 + (self.y_max - y) / (self.y_max - self.y_min) * self.height as f64)
                as f32,
        ))
    }

    pub fn x_ticks(&self) -> Vec<f64> {
        ticks(self.x_min, self.x_max)
    }

    pub fn y_ticks(&self) -> Vec<f64> {
        ticks(self.y_min, self.y_max)
    }
}

/// Choose readable tick positions near ten intervals across a range.
fn ticks(min: f64, max: f64) -> Vec<f64> {
    let rough = (max - min) / 10.0;
    if !rough.is_finite() || rough <= 0.0 {
        return Vec::new();
    }
    let magnitude = 10_f64.powf(rough.log10().floor());
    let scaled = rough / magnitude;
    let multiple = if scaled <= 1.0 {
        1.0
    } else if scaled <= 2.0 {
        2.0
    } else if scaled <= 5.0 {
        5.0
    } else {
        10.0
    };
    let step = multiple * magnitude;
    if !step.is_finite() || step <= 0.0 {
        return Vec::new();
    }
    let mut result = Vec::new();
    let mut index = (min / step).ceil();
    while index * step <= max && result.len() < 100 {
        let value = index * step;
        result.push(if value == 0.0 { 0.0 } else { value });
        index += 1.0;
    }
    result
}
