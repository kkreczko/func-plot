use crate::tokenize::Token;

pub fn convert_to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token>;

    tokens
    // for token in &tokens {
    //     if token == Token::TokNum || token == Token::TokVar {}
    // }
}
