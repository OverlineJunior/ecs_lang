use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Debug, Error)]
pub enum LexError {
    #[error("[line {}] Unexpected character: `{}`", line, .ch)]
    UnexpectedChar { ch: char, line: usize },
    #[error("[line {}] Digit expected after dot", .line)]
    ExpectedDigitAfterDot { line: usize },
    #[error("[line {}] Unterminated string", .line)]
    UnterminatedString { line: usize },
}
