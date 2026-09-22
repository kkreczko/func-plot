use crate::tokenize::Token;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownToken,
    UnmatchedOpenParen,
    UnmatchedCloseParen,
    MissingOperand,
    MissingOperator,
    FunctionNeedsParentheses,
}

pub fn convert_to_rpn(tokens: &[Token]) -> Result<Vec<Token>, ParseError> {
    let tokens: Vec<&Token> = tokens
        .iter()
        .filter(|token| **token != Token::TokWhitespace)
        .collect();
    let mut output = Vec::new();
    let mut operators: Vec<Token> = Vec::new();
    let mut needs_operand = true;

    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::TokNum(_) | Token::TokVar | Token::TokEuler | Token::TokPi => {
                if !needs_operand {
                    return Err(ParseError::MissingOperator);
                }
                output.push((*token).clone());
                needs_operand = false;
            }
            token if token.is_unary_function() => {
                if !needs_operand {
                    return Err(ParseError::MissingOperator);
                }
                if !matches!(tokens.get(index + 1), Some(Token::TokParenOpen)) {
                    return Err(ParseError::FunctionNeedsParentheses);
                }
                operators.push((*token).clone());
            }
            Token::TokParenOpen => {
                if !needs_operand {
                    return Err(ParseError::MissingOperator);
                }
                operators.push(Token::TokParenOpen);
            }
            Token::TokParenClose => {
                if !operators.contains(&Token::TokParenOpen) {
                    return Err(ParseError::UnmatchedCloseParen);
                }
                if needs_operand {
                    return Err(ParseError::MissingOperand);
                }
                while !matches!(operators.last(), Some(Token::TokParenOpen)) {
                    output.push(operators.pop().unwrap());
                }
                operators.pop();
                if operators.last().is_some_and(Token::is_unary_function) {
                    output.push(operators.pop().unwrap());
                }
                needs_operand = false;
            }
            Token::TokPlus | Token::TokMinus if needs_operand => {
                if **token == Token::TokMinus {
                    operators.push(Token::TokNeg);
                }
            }
            token if token.is_operator() => {
                if needs_operand {
                    return Err(ParseError::MissingOperand);
                }
                let order = token.get_operator_order().unwrap();
                while operators.last().is_some_and(|top| {
                    top.get_operator_order().is_some_and(|top_order| {
                        top_order > order || (top_order == order && token.is_left_associated())
                    })
                }) {
                    output.push(operators.pop().unwrap());
                }
                operators.push((*token).clone());
                needs_operand = true;
            }
            _ => return Err(ParseError::UnknownToken),
        }
    }

    if needs_operand {
        return Err(ParseError::MissingOperand);
    }
    while let Some(token) = operators.pop() {
        if token == Token::TokParenOpen {
            return Err(ParseError::UnmatchedOpenParen);
        }
        output.push(token);
    }
    Ok(output)
}

pub fn is_rpn(expr: &[Token]) -> bool {
    let mut depth = 0;
    for token in expr {
        match token {
            Token::TokNum(_) | Token::TokPi | Token::TokEuler | Token::TokVar => depth += 1,
            operator if operator.is_operator() => {
                if depth < 2 {
                    return false;
                }
                depth -= 1;
            }
            unary if unary.is_unary() => {
                if depth < 1 {
                    return false;
                }
            }
            _ => return false,
        }
    }
    depth == 1
}
