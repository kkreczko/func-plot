use func_plot::tokenize::Token;
use func_plot::{
    eval::evaluate_expression, parse::convert_to_rpn, range::generate_range, range::parse_range,
};
use std::env;

fn main() {
    let mut args = env::args().skip(1);

    let (expr, range): (String, String) = match (args.next(), args.next()) {
        (Some(expr), Some(range)) => (expr, range),
        (Some(expr), None) => (expr, String::from("-10;10;0.5")),
        _ => {
            eprintln!(
                "Usage: func-plot <function> <optional:range>\nExample: func-plot \"10 * x ^ 2 + 22 * x - 10\" \"-10;10;0.5\""
            );
            return;
        }
    };

    let mut arguments: Vec<f64> = Vec::new();
    let mut values: Vec<f64> = Vec::new();

    match parse_range(&range) {
        Ok((min, max, step)) => arguments = generate_range(min, max, step),
        Err(error) => {
            eprint!("ERROR Failed parsing range {error:?} {range:?}");
        }
    };

    let tokenized_expr: Vec<Token> = Token::tokenize_expr(&expr);

    match convert_to_rpn(&tokenized_expr) {
        Ok(rpn_expr) => {
            for argument in &arguments {
                match evaluate_expression(&rpn_expr, *argument) {
                    Ok(result) => values.push(result),
                    Err(error) => {
                        eprintln! {"ERROR Failed evaluating expression {error:?} with expr: \n{rpn_expr:?}"}
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("ERROR Failed rpn conversion {error:?} with expr: \n{tokenized_expr:?}")
        }
    }

    println!("{expr} in {range}");
    println!("{arguments:?}");
    println!("{values:?}");
}
