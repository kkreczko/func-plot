use crate::tokenize::Token;

pub fn evaluate_operation(lval: f64, rval: f64, operator: Token) -> f64 {
    match operator {
        Token::TokPlus => lval + rval,
        Token::TokMinus => lval - rval,
        Token::TokMul => lval * rval,
        Token::TokDiv => lval / rval,
        Token::TokPower => lval.powf(rval),
        _ => 0.0,
    }
}

pub fn evaluate_expression(expr: Vec<Token>, value: f64) -> f64 {
    0.0
}

// generate second vector where x is defined for evaluation
pub fn substitute_variable(expr: &Vec<Token>, value: f64) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    for token in expr {
        match token {
            Token::TokVar => result.push(Token::TokNum(value)),
            other => result.push(other.clone()),
        };
    }

    result
}
