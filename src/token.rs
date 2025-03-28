use std::fmt::{self, Display, Formatter};

pub type Spanned<T> = (usize, T, usize);

#[derive(Debug, Clone)]
pub enum Token {
    Number { literal: f64 },
	String { literal: String },
    Boolean { literal: bool },
}

impl Token {
    pub fn lexeme(&self) -> String {
        match self {
            Token::Number { literal } => literal.to_string(),
            Token::String { literal } => literal.clone(),
            Token::Boolean { literal } => literal.to_string(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Token::Number { literal } => write!(f, "{}", literal),
            Token::String { literal } => write!(f, "{}", literal),
            Token::Boolean { literal } => write!(f, "{}", literal),
        }
    }
}
