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
        _ => unreachable!("evaluate_operation needs a binary operator"),
    }
}

pub fn evaluate_expression(expr: &[Token], value: f64) -> Result<f64, EvalErr> {
    if !is_rpn(expr) {
        return Err(EvalErr::EvaluationOnNonRpnExpression);
    }

    let mut stack = Vec::new();
    for token in expr {
        match token {
            Token::TokNum(number) => stack.push(*number),
            Token::TokVar => stack.push(value),
            Token::TokPi => stack.push(std::f64::consts::PI),
            Token::TokEuler => stack.push(std::f64::consts::E),
            operator if operator.is_operator() => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(evaluate_operation(left, right, operator.clone()));
            }
            unary if unary.is_unary() => {
                let operand: f64 = stack.pop().unwrap();
                let result = match unary {
                    Token::TokNeg => -operand,
                    Token::TokSin => operand.sin(),
                    Token::TokCos => operand.cos(),
                    Token::TokLog => operand.ln(),
                    Token::TokSqrt => operand.sqrt(),
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            _ => unreachable!("is_rpn rejects this token"),
        }
    }
    Ok(stack.pop().unwrap())
}

pub fn substitute_variables_and_constants(expr: &[Token], value: f64) -> Vec<Token> {
    expr.iter()
        .map(|token| match token {
            Token::TokVar => Token::TokNum(value),
            Token::TokPi => Token::TokNum(std::f64::consts::PI),
            Token::TokEuler => Token::TokNum(std::f64::consts::E),
            other => other.clone(),
        })
        .collect()
}
