use std::{iter::Peekable, str::Chars};
use crate::{lex_error::LexError, token::{Spanned, Token}};

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
			'0'..='9' => self.next_number()?,
			'"' => self.next_string()?,

			'\n' => {
				self.chars.next();
				self.line += 1;
				return self.next_token();
			}

			c if is_whitespace(*c) => {
				self.chars.next();
				return self.next_token();
			},
			
			c => return Err(LexError::UnexpectedChar { ch: *c, line: 1 }),
		};

		Ok(Some(token))
	}

	fn next_number(&mut self) -> Result<Spanned<Token>, LexError> {
		let mut lexeme = String::new();

		while let Some(&ch) = self.chars.peek() {
			match ch {
				'0'..='9' => lexeme.push(ch),
				_ => break,
			}

			self.chars.next();
		}

		Ok((
			self.line,
			Token::Number {
				literal: lexeme.parse().expect("Failed to parse number"),
			},
			self.line
		))
	}

	fn next_string(&mut self) -> Result<Spanned<Token>, LexError> {
		let mut lexeme = String::new();
		self.chars.next();

		while let Some(&ch) = self.chars.peek() {
			match ch {
				'"' => {
					self.chars.next();
					break;
				}
				'\n' => {
					return Err(LexError::UnterminatedString {
						line: self.line,
					});
				}
				_ => lexeme.push(ch),
			}

			self.chars.next();
		}

		Ok((
			self.line,
			Token::String {
				literal: lexeme,
			},
			self.line
		))
	}
}
