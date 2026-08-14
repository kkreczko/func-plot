use crate::tokenize::Token;

pub fn convert_to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        if matches!(&token, Token::TokNum(_) | Token::TokVar) {
            output.push(token);
        } else if token.is_operator() {
            while !operator_stack.is_empty() {
                match operator_stack.last() {
                    Some(top) => {
                        let top_order: u8 = top.get_operator_order().unwrap();
                        let token_order: u8 = token.get_operator_order().unwrap();
                        if top_order > token_order {
                            output.push(operator_stack.pop().unwrap());
                        } else if top_order == token_order && token.is_left_associated().unwrap() {
                            output.push(operator_stack.pop().unwrap());
                        }
                    }
                    None => break,
                }
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
