use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Debug, Error)]
pub enum ScanError {
    #[error("[line {}] Unexpected character: `{}`", .span.0, .ch)]
    UnexpectedChar {
        ch: char,
        span: (usize, usize),
    },
	#[error("[line {}] Digit expected after dot", .span.0)]
    ExpectedDigitAfterDot {
        span: (usize, usize),
    },
	#[error("[line {}] Unterminated string", .span.0)]
    UnterminatedString {
        span: (usize, usize),
    },
}
