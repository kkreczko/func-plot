pub mod tokenize;

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
    println!("{:?}", Token::tokenize_expr(&expr));
}
