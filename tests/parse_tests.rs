use func_plot::parse::convert_to_rpn;
use func_plot::tokenize::Token;

fn rpn(expression: &str) -> Vec<Token> {
    convert_to_rpn(Token::tokenize_expr(expression))
}

#[test]
fn converts_simple_addition() {
    assert_eq!(
        rpn("2 + 3"),
        vec![Token::TokNum(2.0), Token::TokNum(3.0), Token::TokPlus]
    );
}

#[test]
fn multiplication_has_higher_precedence_than_addition() {
    assert_eq!(
        rpn("2 + 3 * x"),
        vec![
            Token::TokNum(2.0),
            Token::TokNum(3.0),
            Token::TokVar,
            Token::TokMul,
            Token::TokPlus,
        ]
    );
}

#[test]
fn subtraction_is_left_associative() {
    assert_eq!(
        rpn("10 - 3 - 2"),
        vec![
            Token::TokNum(10.0),
            Token::TokNum(3.0),
            Token::TokMinus,
            Token::TokNum(2.0),
            Token::TokMinus,
        ]
    );
}

#[test]
fn power_is_right_associative() {
    assert_eq!(
        rpn("2 ^ 3 ^ 4"),
        vec![
            Token::TokNum(2.0),
            Token::TokNum(3.0),
            Token::TokNum(4.0),
            Token::TokPower,
            Token::TokPower,
        ]
    );
}

#[test]
fn handles_parenthesized_expression_before_power() {
    assert_eq!(
        rpn("( 10 + x ) ^ 2"),
        vec![
            Token::TokNum(10.0),
            Token::TokVar,
            Token::TokPlus,
            Token::TokNum(2.0),
            Token::TokPower,
        ]
    );
}

#[test]
fn support_no_whitespace_exprs() {
    assert_eq!(
        rpn("1+1"),
        vec![Token::TokNum(1.0), Token::TokPlus, Token::TokNum(1.0),]
    );
}
