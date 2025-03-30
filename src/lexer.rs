use crate::{
    lex_error::LexError,
    token::{Spanned, Token},
};
use std::{iter::Peekable, str::Chars};

pub fn tokenize(source: String) -> Result<Vec<Spanned<Token>>, LexError> {
    let mut lexer = Lexer::new(&source);
    let mut tokens = Vec::new();

    loop {
        match lexer.next_token() {
            Ok(Some(token)) => tokens.push(token),
            Ok(None) => break,
            Err(e) => return Err(e),
        }
    }

    Ok(tokens)
}

fn eat_while<F>(chars: &mut Peekable<Chars>, predicate: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut lexeme = String::new();

    while let Some(&ch) = chars.peek() {
        if predicate(ch) {
            lexeme.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    lexeme
}

fn is_name_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_name_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\r' | '\t')
}

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            line: 1,
        }
    }

    fn next_token(&mut self) -> Result<Option<Spanned<Token>>, LexError> {
        if self.chars.peek().is_none() {
            return Ok(None);
        }

        let token = match self.chars.peek().unwrap() {
            c if c.is_ascii_digit() => self.next_number()?,
            '"' => self.next_string()?,

            c if is_name_start(*c) => self.next_name(),

            '\n' => {
                self.chars.next();
                self.line += 1;
                return self.next_token();
            }

            c if is_whitespace(*c) => {
                self.chars.next();
                return self.next_token();
            }

            c => return Err(LexError::UnexpectedChar { ch: *c, line: 1 }),
        };

        Ok(Some(token))
    }

    fn next_number(&mut self) -> Result<Spanned<Token>, LexError> {
        assert!(
            self.chars.peek().unwrap().is_ascii_digit(),
            "`next_number` should be called only when the current char is a digit"
        );

        let lexeme = eat_while(&mut self.chars, |c| c.is_ascii_digit());
        let literal = lexeme
            .parse::<f64>()
            .unwrap_or_else(|_| panic!("Failed to parse number: {lexeme}"));

        Ok((self.line, Token::Number { literal }, self.line))
    }

    fn next_string(&mut self) -> Result<Spanned<Token>, LexError> {
        assert_eq!(
            self.chars.peek().unwrap(),
            &'"',
            "`next_string` should be called only when the current char is a double quote"
        );

        let mut lexeme = String::new();
        self.chars.next();

        while let Some(&ch) = self.chars.peek() {
            match ch {
                '"' => {
                    self.chars.next();
                    break;
                }
                '\n' => {
                    return Err(LexError::UnterminatedString { line: self.line });
                }
                _ => lexeme.push(ch),
            }

            self.chars.next();
        }

        Ok((self.line, Token::String { literal: lexeme }, self.line))
    }

    /// First attempts to read a keyword, otherwise reads an identifier.
    fn next_name(&mut self) -> Spanned<Token> {
        assert!(
            is_name_start(*self.chars.peek().unwrap()),
            "`next_name` should be called only when the current char is a name start"
        );

        let lexeme = eat_while(&mut self.chars, |c| is_name_start(c) || is_name_continue(c));

        let token = Token::keyword_from(&lexeme).unwrap_or(Token::Identifier {
            name: lexeme.clone(),
        });

        (self.line, token, self.line)
    }
}
