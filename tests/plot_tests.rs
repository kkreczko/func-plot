use func_plot::draw::PlotArea;
use func_plot::eval::evaluate_expression;
use func_plot::parse::{ParseError, convert_to_rpn};
use func_plot::range::{RangeParseError, generate_range, parse_range};
use func_plot::tokenize::Token;

fn eval(source: &str, x: f64) -> f64 {
    let tokens = Token::tokenize_expr(source);
    let rpn = convert_to_rpn(&tokens).unwrap();
    evaluate_expression(&rpn, x).unwrap()
}

#[test]
fn unary_signs_follow_power_precedence() {
    assert_eq!(eval("-x^2", 3.0), -9.0);
    assert_eq!(eval("(-x)^2", 3.0), 9.0);
    assert_eq!(eval("2^-3", 0.0), 0.125);
    assert_eq!(eval("--x + +2", 3.0), 5.0);
}

#[test]
fn functions_and_scientific_numbers_work_without_spaces() {
    assert!((eval("sin(pi/2)+sqrt(4)+log(e)", 0.0) - 4.0).abs() < 1e-12);
    assert!((eval("1e-3*x", 2.0) - 0.002).abs() < 1e-12);
}

#[test]
fn rejects_malformed_expressions() {
    for (source, error) in [
        ("x+", ParseError::MissingOperand),
        ("x(2)", ParseError::MissingOperator),
        ("sin x", ParseError::FunctionNeedsParentheses),
        ("(x", ParseError::UnmatchedOpenParen),
        ("x)", ParseError::UnmatchedCloseParen),
        ("x@2", ParseError::UnknownToken),
    ] {
        assert_eq!(convert_to_rpn(&Token::tokenize_expr(source)), Err(error));
    }
}

#[test]
fn validates_range_and_includes_right_endpoint() {
    assert_eq!(parse_range("-1;1;0.75"), Ok((-1.0, 1.0, 0.75)));
    assert_eq!(generate_range(-1.0, 1.0, 0.75), vec![-1.0, -0.25, 0.5, 1.0]);
    for invalid in [
        "",
        "1;2",
        "1;2;0;4",
        "1;1;0.1",
        "0;1;0",
        "0;1;-1",
        "NaN;1;0.1",
    ] {
        assert_eq!(
            parse_range(invalid),
            Err(RangeParseError::IncorrectRangeString)
        );
    }
}

#[test]
fn maps_multiple_curves_to_a_shared_plot_and_ignores_undefined_values() {
    let curves = vec![
        vec![(-1.0, -1.0), (0.0, 0.0), (1.0, 1.0)],
        vec![(-1.0, f64::NAN), (0.0, 2.0), (1.0, 3.0)],
    ];
    let area = PlotArea::new(1000, 700, 80, -1.0, 1.0, &curves).unwrap();
    assert!(area.y_min < -1.0);
    assert!(area.y_max > 3.0);
    assert_eq!(area.to_pixels(-1.0, area.y_max), Some((72.0, 80.0)));
    assert_eq!(
        area.to_pixels(1.0, area.y_min),
        Some((
            (area.left + area.width) as f32,
            (area.top + area.height) as f32
        ))
    );
    assert_eq!(area.to_pixels(0.0, f64::NAN), None);
    assert!(area.x_ticks().contains(&0.0));
}
