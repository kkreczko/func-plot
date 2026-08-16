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

#[test]
fn converts_sine_to_rpn() {
    assert_eq!(rpn("sin ( x )"), vec![Token::TokVar, Token::TokSin]);
}

#[test]
fn converts_cosine_of_an_expression_to_rpn() {
    assert_eq!(
        rpn("cos ( x + 1 )"),
        vec![
            Token::TokVar,
            Token::TokNum(1.0),
            Token::TokPlus,
            Token::TokCos,
        ]
    );
}

#[test]
fn handles_a_function_before_multiplication() {
    assert_eq!(
        rpn("2 * sin ( x )"),
        vec![
            Token::TokNum(2.0),
            Token::TokVar,
            Token::TokSin,
            Token::TokMul,
        ]
    );
}

#[test]
fn handles_nested_trigonometric_functions() {
    assert_eq!(
        rpn("sin ( cos ( x ) )"),
        vec![Token::TokVar, Token::TokCos, Token::TokSin]
    );
}

#[test]
fn applies_power_after_a_parenthesized_function_call() {
    assert_eq!(
        rpn("sin ( x ) ^ 2"),
        vec![
            Token::TokVar,
            Token::TokSin,
            Token::TokNum(2.0),
            Token::TokPower,
        ]
    );
}

#[test]
fn supports_function_calls_without_whitespace() {
    assert_eq!(rpn("sin(x)"), vec![Token::TokVar, Token::TokSin]);
}

#[test]
fn supports_nested_functions_without_whitespace() {
    assert_eq!(
        rpn("sqrt(1+cos(x))"),
        vec![
            Token::TokNum(1.0),
            Token::TokVar,
            Token::TokCos,
            Token::TokPlus,
            Token::TokSqrt,
        ]
    );
}
