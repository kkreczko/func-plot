use crate::tokenize::Token;

pub fn convert_to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        if matches!(&token, Token::TokNum(_) | Token::TokVar) {
            output.push(token);
        } else if token.is_operator() {
            while !operator_stack.is_empty() {
                if let Some(top) = operator_stack.first()
                    && let Some(top_order) = top.get_operator_order()
                {
                    let token_order: u8 = token.get_operator_order().unwrap();
                    let can_pop: bool = top_order > token_order
                        || top_order == token_order && token.is_left_associated().unwrap();
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
                    panic!("You didnt open one of the parenthesis and you are closing one");
                }
                output.push(operator_stack.pop().unwrap());
            }
            operator_stack.pop();
        } else if token == Token::TokErr {
            panic!("One of the tokens is unknown");
        }
    }

    while !operator_stack.is_empty() {
        let temp_tok: Token = operator_stack.pop().unwrap();
        if matches!(temp_tok, Token::TokParenOpen | Token::TokParenClose) {
            panic!("Too many parenthesis")
        }
        output.push(temp_tok);
    }

    output
}
