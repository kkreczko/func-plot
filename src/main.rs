use func_plot::tokenize::Token;
use func_plot::{eval::evaluate_expression, parse::convert_to_rpn};
use std::env;

fn main() {
    let mut args = env::args().skip(1);

    let (expr, range): (String, String) = match (args.next(), args.next()) {
        (Some(expr), Some(range)) => (expr, range),
        (Some(expr), None) => (expr, String::from("-10;10;0.1")),
        _ => {
            eprintln!(
                "Usage: func-plot <function> <optional:range>\nExample: func-plot \"10 * x ^ 2 + 22 * x - 10\" \"-10;10;0.1\""
            );
            return;
        }
    };

    let tokenized_expr: Vec<Token> = Token::tokenize_expr(&expr);

    match convert_to_rpn(tokenized_expr) {
        Ok(rpn_expr) => {
            let value = evaluate_expression(&rpn_expr, 10.0);
            match value {
                Ok(result) => println!("{result}"),
                Err(error) => {
                    eprintln! {"ERROR Failed evaluating expression {error:?}"}
                }
            }
        }
        Err(error) => {
            eprintln!("ERROR Failed rpn conversion {error:?}")
        }
    }
}
