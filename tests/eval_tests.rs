use func_plot::eval::{
    evaluate_expression, evaluate_operation, substitute_variables_and_constants,
};
use func_plot::tokenize::Token;

fn assert_close(actual: f64, expected: f64) {
    let difference = (actual - expected).abs();
    assert!(
        difference < 1e-12,
        "expected {expected}, got {actual} (difference: {difference})"
    );
}

fn evaluate_successfully(expression: &[Token], value: f64) -> f64 {
    evaluate_expression(expression, value).expect("test expression should evaluate successfully")
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
        substitute_variables_and_constants(&expression, 3.0),
        vec![
            Token::TokNum(3.0),
            Token::TokNum(2.0),
            Token::TokMul,
            Token::TokNum(3.0),
            Token::TokPlus,
        ]
    );
}

#[test]
fn evaluates_a_simple_rpn_expression() {
    let expression = vec![Token::TokNum(2.0), Token::TokNum(3.0), Token::TokPlus];

    assert_eq!(evaluate_successfully(&expression, 0.0), 5.0);
}

#[test]
fn evaluates_an_rpn_expression_with_a_variable() {
    let expression = vec![Token::TokVar, Token::TokNum(2.0), Token::TokPower];

    assert_eq!(evaluate_successfully(&expression, 3.0), 9.0);
}

#[test]
fn evaluates_sine_as_a_unary_function() {
    let expression = vec![Token::TokVar, Token::TokSin];

    assert_close(
        evaluate_successfully(&expression, std::f64::consts::FRAC_PI_2),
        1.0,
    );
}

#[test]
fn evaluates_nested_unary_functions() {
    let expression = vec![Token::TokVar, Token::TokCos, Token::TokSqrt];

    assert_close(evaluate_successfully(&expression, 0.0), 1.0);
}

#[test]
fn evaluates_pi_constant() {
    let expression = vec![Token::TokPi];

    assert_close(evaluate_successfully(&expression, 0.0), 3.14159);
}

#[test]
fn evaluates_e_constant() {
    let expression = vec![Token::TokEuler];

    assert_close(evaluate_successfully(&expression, 0.0), 2.71828);
}
