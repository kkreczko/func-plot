
#[derive(Debug)]
pub enum Token{
    TokErr,
    TokNum(f64),
    TokPlus,
    TokMinus,
    TokDiv,
    TokMul,
    TokParenOpen,
    TokParenClose,
    TokVar,
    TokPower,
}

impl Token {
    pub fn tokenize_word(word: &str) -> Self {
        match word {
            "+" => Self::TokPlus,
            "-" => Self::TokMinus,
            "/" => Self::TokDiv,
            "*" => Self::TokMul,
            "(" => Self::TokParenOpen,
            ")" => Self::TokParenClose,
            "x" => Self::TokVar,
            "^" => Self::TokPower,
            _ => match word.parse::<f64>() {
                Ok(number) => Self::TokNum(number),
                Err(_) => Self::TokErr,
            },
        }
    }

    pub fn tokenize_expr(expr: &str) -> Vec<Self> {
        expr
            .split_whitespace()
            .map(Self::tokenize_word)
            .collect()
    }
}
