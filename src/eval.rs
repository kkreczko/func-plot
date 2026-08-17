use crate::eval::EvalErr::EvaluationOnNonRpnExpression;
use crate::parse::convert_to_rpn;
use crate::parse::is_rpn;
use crate::tokenize::Token;

#[derive(Debug)]
pub enum EvalErr {
    EvaluationOnNonRpnExpression,
}

pub fn evaluate_operation(lval: f64, rval: f64, operator: Token) -> f64 {
    match operator {
        Token::TokPlus => lval + rval,
        Token::TokMinus => lval - rval,
        Token::TokMul => lval * rval,
        Token::TokDiv => lval / rval,
        Token::TokPower => lval.powf(rval),
        _ => 0.0, // something is not yes
    }
}

pub fn evaluate_expression(expr: &[Token], value: f64) -> Result<f64, EvalErr> {
    if !is_rpn(expr) {
        return Err(EvalErr::EvaluationOnNonRpnExpression);
    }

    let expr_with_sub_var: Vec<Token> = substitute_variables_and_constants(expr, value);
    let mut result_stack: Vec<f64> = Vec::new();

    for token in expr_with_sub_var {
        match token {
            Token::TokNum(number) => result_stack.push(number),
            operator if operator.is_operator() => {
                let rval = result_stack.pop().unwrap();
                let lval = result_stack.pop().unwrap();

                let result = evaluate_operation(lval, rval, operator);

                result_stack.push(result);
            }
            _ => {}
        }
    }

    Ok(result_stack.pop().unwrap())
}

// generate second vector where x is defined for evaluation
pub fn substitute_variables_and_constants(expr: &[Token], value: f64) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    for token in expr {
        match token {
            Token::TokVar => result.push(Token::TokNum(value)),
            Token::TokPi => result.push(Token::TokNum(3.14159)),
            Token::TokEuler => result.push(Token::TokNum(2.71828)),
            other => result.push(other.clone()),
        };
    }

    result
}
