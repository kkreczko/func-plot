#[derive(Debug, PartialEq)]
pub enum Token {
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
        expr.split_whitespace().map(Self::tokenize_word).collect()
    }

    pub fn is_operator(&self) -> bool {
        match self {
            Self::TokPlus | Self::TokMinus | Self::TokMul | Self::TokDiv | Self::TokPower => true,
            _ => false,
        }
    }

    pub fn get_operator_order(&self) -> Option<u8> {
        match self {
            Self::TokPower => Some(3),
            Self::TokDiv | Self::TokMul => Some(2),
            Self::TokPlus | Self::TokMinus => Some(1),
            _ => None,
        }
    }

    // None when called not on operator
    pub fn is_left_associated(&self) -> Option<bool> {
        match self {
            Self::TokPlus | Self::TokMinus | Self::TokMul | Self::TokDiv => Some(true),
            Self::TokPower => Some(false),
            _ => None,
        }
    }
}
