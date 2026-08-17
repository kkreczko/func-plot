use crate::tokenize::Token;

#[derive(Debug)]
pub enum ParseError {
    UnknownToken,
    UnmatchedOpenParen,
    UnmatchedCloseParen,
}

pub fn convert_to_rpn(tokens: Vec<Token>) -> Result<Vec<Token>, ParseError> {
    let mut output: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        if matches!(
            &token,
            Token::TokNum(_) | Token::TokVar | Token::TokEuler | Token::TokPi
        ) {
            output.push(token);
        } else if token.is_operator() {
            while !operator_stack.is_empty() {
                if let Some(top) = operator_stack.first()
                    && let Some(top_order) = top.get_operator_order()
                {
                    let token_order: u8 = token.get_operator_order().unwrap();
                    let can_pop: bool = top_order > token_order
                        || top_order == token_order && token.is_left_associated();
                    if can_pop {
                        output.push(operator_stack.pop().unwrap());
                    }
                }
                break;
            }
            operator_stack.push(token);
        } else if matches!(&token, Token::TokParenOpen) {
            operator_stack.push(token);
        } else if matches!(&token, Token::TokParenClose) {
            while !matches!(operator_stack.last().unwrap(), Token::TokParenOpen) {
                if operator_stack.is_empty() {
                    return Err(ParseError::UnmatchedOpenParen);
                }
                output.push(operator_stack.pop().unwrap());
            }
            operator_stack.pop();
        } else if token == Token::TokErr {
            return Err(ParseError::UnknownToken);
        }
    }

    while !operator_stack.is_empty() {
        let temp_tok: Token = operator_stack.pop().unwrap();
        if matches!(temp_tok, Token::TokParenOpen | Token::TokParenClose) {
            return Err(ParseError::UnmatchedCloseParen);
        }
        output.push(temp_tok);
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
            unary_func if unary_func.is_unary_function() => {
                if depth < 1 {
                    return false;
                }
            }
            _ => return false,
        }
    }

    depth == 1
}
