use func_plot::eval::substitute_variable;
use func_plot::parse::convert_to_rpn;
use func_plot::tokenize::Token;
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

    println!("{range}");
    let tokenized_expr: Vec<Token> = Token::tokenize_expr(&expr);
    println!("{:?}", tokenized_expr);
    let rpn_expr: Vec<Token> = convert_to_rpn(tokenized_expr);
    println!("{:?}", rpn_expr);
}
