use func_plot::parse::{convert_to_rpn, is_rpn};
use func_plot::tokenize::Token;

fn rpn(expression: &str) -> Vec<Token> {
    convert_to_rpn(Token::tokenize_expr(expression))
        .expect("test expression should convert to RPN")
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

#[test]
fn recognizes_valid_rpn_with_binary_and_unary_operations() {
    let expression = [
        Token::TokVar,
        Token::TokNum(2.0),
        Token::TokPower,
        Token::TokSin,
    ];

    assert!(is_rpn(&expression));
}

#[test]
fn recognizes_single_operands_as_valid_rpn() {
    for operand in [
        Token::TokNum(2.0),
        Token::TokVar,
        Token::TokPi,
        Token::TokEuler,
    ] {
        assert!(is_rpn(&[operand.clone()]), "rejected operand: {operand:?}");
    }
}

#[test]
fn rejects_an_empty_expression() {
    assert!(!is_rpn(&[]));
}

#[test]
fn rejects_an_operator_without_enough_operands() {
    let expression = [Token::TokNum(2.0), Token::TokPlus];

    assert!(!is_rpn(&expression));
}

#[test]
fn rejects_a_unary_function_without_an_operand() {
    assert!(!is_rpn(&[Token::TokSin]));
}

#[test]
fn rejects_leftover_operands() {
    let expression = [Token::TokNum(2.0), Token::TokNum(3.0)];

    assert!(!is_rpn(&expression));
}

#[test]
fn rejects_tokens_that_do_not_belong_in_rpn() {
    for token in [
        Token::TokParenOpen,
        Token::TokParenClose,
        Token::TokWhitespace,
        Token::TokErr,
    ] {
        let expression = [Token::TokNum(2.0), token];
        assert!(!is_rpn(&expression));
    }
}
