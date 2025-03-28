/*
LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    True,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,
    Eof,
    Question,
    Colon,
*/

use std::fmt::{self, Display, Formatter};

pub type Spanned<T> = (usize, T, usize);

#[derive(Debug, Clone)]
pub enum Token {
    Number { literal: f64 },
	String { literal: String },
    Boolean { literal: bool },
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
