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
            eprintln!("gib command");
            return;
        }
    };

    println!("{range}");
    let tokenized_expr: Vec<Token> = Token::tokenize_expr(&expr);
    println!("{:?}", tokenized_expr);
    let change_x: Vec<Token> = substitute_variable(&tokenized_expr, 11.0);
    println!("{:?}", change_x);
    let rpn_expr: Vec<Token> = convert_to_rpn(tokenized_expr);
    println!("{:?}", rpn_expr);
}
