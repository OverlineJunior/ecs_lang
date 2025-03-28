pub mod token;
pub mod lex_error;
pub mod lexer;

use token::Token;

fn main() {
	let src = "123\n   \"hello\"".to_string();
	let tokens = lexer::tokenize(src).unwrap();

	tokens.iter().for_each(|token| {
		match token {
			(s, Token::Number { literal }, e) => println!("{s}..{e} Number: {literal}"),
			(s, Token::String { literal }, e) => println!("{s}..{e} String: {literal}"),
			(s, Token::Boolean { literal }, e) => println!("{s}..{e} Boolean: {literal}"),
		}
	});
}
