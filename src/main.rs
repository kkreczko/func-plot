use func_plot::draw::PlotArea;
use func_plot::eval::evaluate_expression;
use func_plot::parse::convert_to_rpn;
use func_plot::range::{generate_range, parse_range};
use func_plot::tokenize::Token;
use raylib::prelude::*;
use std::env;
use std::io::{self, BufRead, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

const WIDTH: i32 = 1100;
const HEIGHT: i32 = 720;
const COLORS: [Color; 12] = [
    Color::new(255, 105, 120, 255),
    Color::new(80, 200, 255, 255),
    Color::new(255, 210, 80, 255),
    Color::new(120, 230, 145, 255),
    Color::new(200, 145, 255, 255),
    Color::new(255, 155, 95, 255),
    Color::new(95, 225, 210, 255),
    Color::new(235, 135, 210, 255),
    Color::new(190, 220, 85, 255),
    Color::new(255, 145, 175, 255),
    Color::new(115, 155, 255, 255),
    Color::new(205, 185, 135, 255),
];

fn usage() -> &'static str {
    "Usage: func-plot [expression ...] [--range min;max;step]\n\
     Enter one expression per line in the terminal to add it to the plot. Type :quit to exit.\n\
     Example: func-plot --range \"-10;10;0.02\"\n\
     Functions: sin, cos, sqrt, log (natural logarithm). Constants: pi, e."
}

fn parse_args() -> Result<Option<(Vec<String>, String)>, String> {
    let mut args = env::args().skip(1);
    let mut expressions = Vec::new();
    let mut range = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(None),
            "--range" => {
                if range.is_some() {
                    return Err("range was specified more than once".into());
                }
                range = Some(args.next().ok_or("--range needs min;max;step")?);
            }
            _ if arg.contains(';') => {
                if range.is_some() {
                    return Err("range was specified more than once".into());
                }
                range = Some(arg);
            }
            _ if arg.starts_with("--") => return Err(format!("unknown option: {arg}")),
            _ => expressions.push(arg),
        }
    }
    if expressions.len() > 12 {
        return Err("plot at most 12 expressions at a time".into());
    }
    Ok(Some((
        expressions,
        range.unwrap_or_else(|| "-10;10;0.02".into()),
    )))
}

fn read_expressions(sender: Sender<String>, acknowledged: Receiver<()>) {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    loop {
        print!("f(x)> ");
        if io::stdout().flush().is_err() {
            break;
        }
        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if sender.send(line).is_err() || acknowledged.recv().is_err() {
                    break;
                }
            }
            Err(error) => {
                eprintln!("Could not read from terminal: {error}");
                break;
            }
        }
    }
}

fn add_expression(
    source: &str,
    arguments: &[f64],
    expressions: &mut Vec<String>,
    curves: &mut Vec<Vec<(f64, f64)>>,
) -> Result<(), String> {
    if expressions.len() >= 12 {
        return Err("plot at most 12 expressions at a time".into());
    }
    let tokens = Token::tokenize_expr(source);
    let rpn = convert_to_rpn(&tokens).map_err(|error| format!("{source:?}: {error:?}"))?;
    let curve: Vec<(f64, f64)> = arguments
        .iter()
        .map(|&x| {
            (
                x,
                evaluate_expression(&rpn, x).expect("parser generated valid RPN"),
            )
        })
        .collect();
    if !curve.iter().any(|(_, y)| y.is_finite()) {
        return Err(format!("{source:?} is undefined over this range"));
    }
    expressions.push(source.to_owned());
    curves.push(curve);
    Ok(())
}

fn handle_line(
    line: &str,
    arguments: &[f64],
    expressions: &mut Vec<String>,
    curves: &mut Vec<Vec<(f64, f64)>>,
) -> bool {
    let source = line.trim();
    if source == ":quit" {
        return false;
    }
    if !source.is_empty() {
        match add_expression(source, arguments, expressions, curves) {
            Ok(()) => println!("Added: {source}"),
            Err(error) => eprintln!("{error}"),
        }
    }
    true
}

fn plot_area(min: f64, max: f64, expressions: &[String], curves: &[Vec<(f64, f64)>]) -> PlotArea {
    let top = 52 + expressions.len().div_ceil(3) as i32 * 24;
    PlotArea::new(WIDTH, HEIGHT, top, min, max, curves)
        .expect("at least one curve has finite values")
}

fn tick_label(value: f64) -> String {
    if value != 0.0 && !(0.001..10_000.0).contains(&value.abs()) {
        format!("{value:.1e}")
    } else {
        format!("{value:.3}")
    }
}

fn point(coords: (f32, f32)) -> Vector2 {
    Vector2::new(coords.0, coords.1)
}

fn render(
    d: &mut RaylibDrawHandle<'_>,
    area: &PlotArea,
    expressions: &[String],
    curves: &[Vec<(f64, f64)>],
) {
    let background = Color::new(16, 20, 29, 255);
    let grid = Color::new(49, 57, 70, 255);
    let axis = Color::new(155, 167, 181, 255);
    let text = Color::new(222, 229, 237, 255);
    d.clear_background(background);
    d.draw_text("func-plot", 20, 13, 21, text);

    for (index, expression) in expressions.iter().enumerate() {
        let x = 22 + (index % 3) as i32 * 350;
        let y = 44 + (index / 3) as i32 * 24;
        let color = COLORS[index % COLORS.len()];
        d.draw_line_ex(
            Vector2::new(x as f32, (y + 7) as f32),
            Vector2::new((x + 22) as f32, (y + 7) as f32),
            3.0,
            color,
        );
        let label: String = expression.chars().take(37).collect();
        d.draw_text(&label, x + 31, y, 17, text);
    }

    let right = area.left + area.width;
    let bottom = area.top + area.height;
    for x in area.x_ticks() {
        let px = area.to_pixels(x, area.y_min).unwrap().0.round() as i32;
        d.draw_line(px, area.top, px, bottom, grid);
        let label = tick_label(x);
        let label_width = d.measure_text(&label, 14);
        d.draw_text(&label, px - label_width / 2, bottom + 8, 14, axis);
    }
    for y in area.y_ticks() {
        let py = area.to_pixels(area.x_min, y).unwrap().1.round() as i32;
        d.draw_line(area.left, py, right, py, grid);
        let label = tick_label(y);
        let label_width = d.measure_text(&label, 14);
        d.draw_text(&label, area.left - label_width - 8, py - 7, 14, axis);
    }
    if area.x_min <= 0.0 && area.x_max >= 0.0 {
        let x = area.to_pixels(0.0, area.y_min).unwrap().0.round() as i32;
        d.draw_line_ex(
            Vector2::new(x as f32, area.top as f32),
            Vector2::new(x as f32, bottom as f32),
            2.0,
            axis,
        );
    }
    if area.y_min <= 0.0 && area.y_max >= 0.0 {
        let y = area.to_pixels(area.x_min, 0.0).unwrap().1.round() as i32;
        d.draw_line_ex(
            Vector2::new(area.left as f32, y as f32),
            Vector2::new(right as f32, y as f32),
            2.0,
            axis,
        );
    }

    d.draw_scissor_mode(
        area.left,
        area.top,
        area.width,
        area.height,
        |mut clipped| {
            for (index, curve) in curves.iter().enumerate() {
                let color = COLORS[index % COLORS.len()];
                for pair in curve.windows(2) {
                    let (a, b) = (pair[0], pair[1]);
                    let (Some(first), Some(second)) =
                        (area.to_pixels(a.0, a.1), area.to_pixels(b.0, b.1))
                    else {
                        continue;
                    };
                    if !first.1.is_finite()
                        || !second.1.is_finite()
                        || (first.1 - second.1).abs() > area.height as f32 * 0.8
                    {
                        continue;
                    }
                    clipped.draw_line_ex(point(first), point(second), 2.0, color);
                }
            }
        },
    );
    d.draw_rectangle_lines(area.left, area.top, area.width, area.height, axis);
}

fn run() -> Result<(), String> {
    let Some((initial_expressions, range)) = parse_args()? else {
        println!("{}", usage());
        return Ok(());
    };
    let (min, max, step) = parse_range(&range).map_err(|_| format!("invalid range: {range:?}"))?;
    let sample_count = (max - min) / step;
    if !sample_count.is_finite() || sample_count > 9_998.0 {
        return Err("range contains too many samples (limit: 10,000); increase the step".into());
    }
    let arguments = generate_range(min, max, step);
    let mut expressions = Vec::new();
    let mut curves = Vec::new();
    for expression in initial_expressions {
        add_expression(&expression, &arguments, &mut expressions, &mut curves)?;
    }

    println!("Enter one function per line (:quit to exit).");
    let (line_sender, lines) = mpsc::channel();
    let (acknowledged, acknowledgements) = mpsc::channel();
    thread::spawn(move || read_expressions(line_sender, acknowledgements));

    while expressions.is_empty() {
        let line = lines
            .recv()
            .map_err(|_| "no function was entered before terminal input ended")?;
        let continue_running = handle_line(&line, &arguments, &mut expressions, &mut curves);
        if !continue_running {
            return Ok(());
        }
        let _ = acknowledged.send(());
    }
    let mut area = plot_area(min, max, &expressions, &curves);

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("func-plot")
        .build();
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        for line in lines.try_iter() {
            let continue_running = handle_line(&line, &arguments, &mut expressions, &mut curves);
            if !continue_running {
                return Ok(());
            }
            let _ = acknowledged.send(());
            area = plot_area(min, max, &expressions, &curves);
        }
        let mut drawing = rl.begin_drawing(&thread);
        render(&mut drawing, &area, &expressions, &curves);
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}\n{}", usage());
        std::process::exit(1);
    }
}
