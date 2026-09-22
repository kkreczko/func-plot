#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    TokErr,
    TokNum(f64),
    TokPlus,
    TokMinus,
    TokNeg,
    TokDiv,
    TokMul,
    TokParenOpen,
    TokParenClose,
    TokVar,
    TokPower,
    TokWhitespace,
    TokSin,
    TokCos,
    TokLog,
    TokSqrt,
    TokEuler,
    TokPi,
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
            " " => Self::TokWhitespace,
            "cos" => Self::TokCos,
            "sin" => Self::TokSin,
            "log" => Self::TokLog,
            "sqrt" => Self::TokSqrt,
            "e" => Self::TokEuler,
            "pi" => Self::TokPi,
            _ => word.parse::<f64>().map_or(Self::TokErr, Self::TokNum),
        }
    }

    pub fn tokenize_expr(expr: &str) -> Vec<Self> {
        let chars: Vec<char> = expr.chars().collect();
        let mut tokens = Vec::new();
        let mut i = 0;
        while i < chars.len() {
            let ch = chars[i];
            if ch.is_whitespace() {
                i += 1;
            } else if ch.is_ascii_digit() || ch == '.' {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                if i < chars.len() && matches!(chars[i], 'e' | 'E') {
                    let exponent = i;
                    i += 1;
                    if i < chars.len() && matches!(chars[i], '+' | '-') {
                        i += 1;
                    }
                    let digits = i;
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        i += 1;
                    }
                    if digits == i {
                        i = exponent;
                    }
                }
                let word: String = chars[start..i].iter().collect();
                tokens.push(Self::tokenize_word(&word));
            } else if ch.is_alphabetic() {
                let start = i;
                while i < chars.len() && chars[i].is_alphabetic() {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                tokens.push(Self::tokenize_word(&word));
            } else {
                tokens.push(Self::tokenize_word(&ch.to_string()));
                i += 1;
            }
        }
        tokens
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::TokPlus | Self::TokMinus | Self::TokMul | Self::TokDiv | Self::TokPower
        )
    }

    pub fn get_operator_order(&self) -> Option<u8> {
        match self {
            Self::TokPower => Some(4),
            Self::TokNeg => Some(3),
            Self::TokDiv | Self::TokMul => Some(2),
            Self::TokPlus | Self::TokMinus => Some(1),
            _ => None,
        }
    }

    pub fn is_left_associated(&self) -> bool {
        matches!(
            self,
            Self::TokPlus | Self::TokMinus | Self::TokMul | Self::TokDiv
        )
    }

    pub fn is_unary_function(&self) -> bool {
        matches!(
            self,
            Self::TokSin | Self::TokCos | Self::TokLog | Self::TokSqrt
        )
    }

    pub fn is_unary(&self) -> bool {
        matches!(self, Self::TokNeg) || self.is_unary_function()
    }
}
