
#[derive(Debug)]
pub enum Token{
    TokErr,
    TokVal,
}

impl Token {
    pub fn tokenize(input_string: &str) -> Self {
        match input_string {
            "val" => Self::TokVal,
            _ => Self::TokErr
        }
    }
}
