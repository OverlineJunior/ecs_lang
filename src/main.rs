pub mod lex_error;
pub mod lexer;
pub mod token;

fn main() {
    let src = "123\n   \"hello\"   print".to_string();
    let tokens = lexer::tokenize(src).unwrap();

    tokens
        .iter()
        .for_each(|t| println!("[{}..{}] {}", t.0, t.2, t.1));
}
