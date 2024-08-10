use std::{error::Error, fmt::Display};

use super::{lexer::TokenKind, Span};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    ExpectingButGotEOF(TokenKind),
    UnexpectedChar(char, Span),
    UnexpectedPrimaryExpr(TokenKind, Span),
    Expecting(TokenKind, TokenKind, Span),
    InvalidFunctionCall(Span),
    ParseFloatError(std::num::ParseFloatError, Span),
    ParseIntError(std::num::ParseIntError, Span),
}

impl ParseError {
    pub fn span(&self) -> Option<Span> {
        match self {
            ParseError::UnexpectedEOF => None,
            ParseError::ExpectingButGotEOF(_) => None,
            ParseError::UnexpectedChar(_, span) => Some(*span),
            ParseError::UnexpectedPrimaryExpr(_, span) => Some(*span),
            ParseError::Expecting(_, _, span) => Some(*span),
            ParseError::InvalidFunctionCall(span) => Some(*span),
            ParseError::ParseFloatError(_, span) => Some(*span),
            ParseError::ParseIntError(_, span) => Some(*span),
        }
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedEOF => write!(f, "Unexpected end of file"),
            ParseError::ExpectingButGotEOF(tk) => {
                write!(f, "Expecting '{}' but reach end of file", tk.to_char())
            }
            ParseError::UnexpectedChar(c, _) => {
                write!(f, "Unexpected character '{c}' appear in expression")
            }
            ParseError::UnexpectedPrimaryExpr(tk, _) => {
                write!(f, "Expecting an expression but got '{}'", tk.to_char())
            }
            ParseError::Expecting(ex, got, _) => write!(
                f,
                "Expecting '{}' but got '{}'",
                ex.to_char(),
                got.to_char()
            ),
            ParseError::InvalidFunctionCall(_) => write!(f, "This is not a valid function call"),
            ParseError::ParseFloatError(err, _) => write!(f, "Parse float error: {err}"),
            ParseError::ParseIntError(err, _) => write!(f, "Parse int error: {err}"),
        }
    }
}
