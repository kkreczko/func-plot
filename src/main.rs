pub mod parse;
pub mod tokenize;

use parse::convert_to_rpn;
use std::env;
use tokenize::Token;

fn main() {
    let mut args = env::args().skip(1);

    let (expr, range): (String, String) = match (args.next(), args.next()) {
        (Some(expr), Some(range)) => (expr, range),
        (Some(expr), None) => (expr, String::from("-10;10;0.1")),
        _ => {
            eprintln!("gib command");
            return;
        }
    };

    println!("{range}");
    let tokenized_expr: Vec<Token> = Token::tokenize_expr(&expr);
    let rpn_expr: Vec<Token> = convert_to_rpn(tokenized_expr);
    println!("{:?}", rpn_expr);
}
