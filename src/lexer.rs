use std::{iter::Peekable, str::Chars};

use crate::token::{Spanned, Token};

pub fn tokenize(source: String) -> Result<Vec<Spanned<Token>>, ()> {
	let mut chars = source.chars().peekable();
	let mut tokens = Vec::new();

	loop {
		match eat_token(&mut chars) {
			Ok(Some(token)) => tokens.push(token),
			Ok(None) => break,
			Err(_) => return Err(()),
		}
	}

	Ok(tokens)
}

fn eat_token(chars: &mut Peekable<Chars>) -> Result<Option<Spanned<Token>>, ()> {
	if chars.peek().is_none() {
		return Ok(None);
	}

	let token = match chars.peek().unwrap() {
		'0'..='9' => eat_number(chars)?,
		'"' => eat_string(chars)?,
		_ => return Err(()),
	};

	Ok(Some(token))
}

fn eat_number(chars: &mut Peekable<Chars>) -> Result<Spanned<Token>, ()> {
	let mut lexeme = String::new();

	while let Some(&ch) = chars.peek() {
		match ch {
			'0'..='9' => lexeme.push(ch),
			_ => break,
		}

		chars.next();
	}

	Ok((
		0,
		Token::Number {
			literal: lexeme.parse().expect("Failed to parse number"),
		},
		0
	))
}

fn eat_string(chars: &mut Peekable<Chars>) -> Result<Spanned<Token>, ()> {
	let mut lexeme = String::new();
	chars.next();

	while let Some(&ch) = chars.peek() {
		match ch {
			'"' => {
				chars.next();
				break;
			}
			_ => lexeme.push(ch),
		}

		chars.next();
	}

	Ok((
		0,
		Token::String {
			literal: lexeme,
		},
		0
	))
}
