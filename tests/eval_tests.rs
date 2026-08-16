use func_plot::eval::{evaluate_expression, evaluate_operation, substitute_variable};
use func_plot::tokenize::Token;

fn assert_close(actual: f64, expected: f64) {
    let difference = (actual - expected).abs();
    assert!(
        difference < 1e-12,
        "expected {expected}, got {actual} (difference: {difference})"
    );
}

#[test]
fn evaluates_addition() {
    assert_eq!(evaluate_operation(2.0, 3.0, Token::TokPlus), 5.0);
}

#[test]
fn evaluates_subtraction_in_operand_order() {
    assert_eq!(evaluate_operation(10.0, 3.0, Token::TokMinus), 7.0);
}

#[test]
fn evaluates_multiplication() {
    assert_eq!(evaluate_operation(4.0, 2.5, Token::TokMul), 10.0);
}

#[test]
fn evaluates_division_in_operand_order() {
    assert_eq!(evaluate_operation(9.0, 3.0, Token::TokDiv), 3.0);
}

#[test]
fn evaluates_fractional_powers() {
    assert_close(evaluate_operation(9.0, 0.5, Token::TokPower), 3.0);
}

#[test]
fn substitutes_every_occurrence_of_the_variable() {
    let expression = vec![
        Token::TokVar,
        Token::TokNum(2.0),
        Token::TokMul,
        Token::TokVar,
        Token::TokPlus,
    ];

    assert_eq!(
        substitute_variable(&expression, 3.0),
        vec![
            Token::TokNum(3.0),
            Token::TokNum(2.0),
            Token::TokMul,
            Token::TokNum(3.0),
            Token::TokPlus,
        ]
    );
}

// evaluate_expression receives tokens in Reverse Polish Notation. These tests
// describe the stack evaluator that will consume that representation.
#[test]
fn evaluates_a_simple_rpn_expression() {
    let expression = vec![Token::TokNum(2.0), Token::TokNum(3.0), Token::TokPlus];

    assert_eq!(evaluate_expression(expression, 0.0), 5.0);
}

#[test]
fn evaluates_an_rpn_expression_with_a_variable() {
    let expression = vec![Token::TokVar, Token::TokNum(2.0), Token::TokPower];

    assert_eq!(evaluate_expression(expression, 3.0), 9.0);
}

#[test]
fn evaluates_sine_as_a_unary_function() {
    let expression = vec![Token::TokVar, Token::TokSin];

    assert_close(
        evaluate_expression(expression, std::f64::consts::FRAC_PI_2),
        1.0,
    );
}

#[test]
fn evaluates_nested_unary_functions() {
    let expression = vec![Token::TokVar, Token::TokCos, Token::TokSqrt];

    assert_close(evaluate_expression(expression, 0.0), 1.0);
}
