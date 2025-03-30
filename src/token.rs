use std::fmt::{self, Display, Formatter};

pub type Spanned<T> = (usize, T, usize);

fn uppercase_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Number { literal: f64 },
    String { literal: String },
    Boolean { literal: bool },
    Identifier { name: String },
    None,
    Print,
}

impl Token {
    // Cannot be exhaustive, thus never forget to add new keywords here.
    pub fn keyword_from(s: &str) -> Option<Token> {
        match s {
            "true" => Some(Token::Boolean { literal: true }),
            "false" => Some(Token::Boolean { literal: false }),
            "none" => Some(Token::None),
            "print" => Some(Token::Print),
            _ => None,
        }
    }

    pub fn lexeme(&self) -> String {
        match self {
            Token::Number { literal } => literal.to_string(),
            Token::String { literal } => literal.clone(),
            Token::Boolean { literal } => literal.to_string(),
            Token::Identifier { name } => name.clone(),
            Token::None => "none".to_string(),
            Token::Print => "print".to_string(),
        }
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Token::Number { .. } | Token::String { .. } | Token::Boolean { .. }
        )
    }

    pub fn is_statement(&self) -> bool {
        matches!(self, Token::Print)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Token::Number { literal } => write!(f, "Number({})", literal),
            Token::String { literal } => write!(f, "String({})", literal),
            Token::Boolean { literal } => write!(f, "Boolean({})", literal),
            Token::Identifier { name } => write!(f, "Identifier({})", name),
            _ => uppercase_first(&self.lexeme()).fmt(f),
        }
    }
}
