
#[derive(Debug)]
pub enum Token{
    TokErr,
    TokNum(f64),
    TokPlus,
    TokMinus,
    TokDiv,
    TokMul,

}

impl Token {
    // todo should return token and value for numbers
    pub fn tokenize_word(word: &str) -> Self {
        match word {
            "num" => Self::TokNum(10.0), // todo change to numeric string
            "+" => Self::TokPlus,
            "-" => Self::TokMinus,
            "/" => Self::TokDiv,
            "*" => Self::TokMul,
            _ => Self::TokErr
        }
    }

    // pub fn tokenize_expr(expr: &str) -> Vec<Token> {
    //
    // }
}
