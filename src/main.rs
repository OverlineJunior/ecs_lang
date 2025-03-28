pub mod token;
pub mod lexical_error;
pub mod lexer;

use token::{Spanned, Token};

fn main() {
	let tokens: [Spanned<Token>; 3] = [
		(0, Token::Number { literal: 123. }, 1),
		(1, Token::String { literal: "hello".to_string() }, 2),
		(2, Token::Boolean { literal: true }, 3),
	];

	tokens.iter().for_each(|token| {
		match token {
			(s, Token::Number { literal }, e) => println!("{s}..{e} Number: {literal}"),
			(s, Token::String { literal }, e) => println!("{s}..{e} String: {literal}"),
			(s, Token::Boolean { literal }, e) => println!("{s}..{e} Boolean: {literal}"),
		}
	});
}
