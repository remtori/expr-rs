use std::{backtrace::Backtrace, error::Error, fmt::Display};

use super::{lexer::TokenKind, Span};

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedEOF,
    ExpectingButGotEOF(TokenKind),
    UnexpectedChar(char),
    UnexpectedPrimaryExpr(TokenKind),
    UnexpectedTokenAtEOF(TokenKind),
    Expecting(TokenKind, TokenKind),
    InvalidFunctionCall,
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    span: Option<Span>,
    backtrace: Backtrace,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, span: Span) -> Self {
        Self {
            kind,
            span: Some(span),
            backtrace: Backtrace::capture(),
        }
    }

    pub fn new_nospan(kind: ParseErrorKind) -> Self {
        Self {
            kind,
            span: None,
            backtrace: Backtrace::capture(),
        }
    }

    pub fn span(&self) -> Option<Span> {
        self.span
    }

    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ParseErrorKind::UnexpectedEOF => write!(f, "Unexpected end of file"),
            ParseErrorKind::ExpectingButGotEOF(tk) => {
                write!(f, "Expecting '{}' but reach end of file", tk.to_char())
            }
            ParseErrorKind::UnexpectedChar(c) => {
                write!(f, "Unexpected character '{c}' appear in expression")
            }
            ParseErrorKind::UnexpectedPrimaryExpr(tk) => {
                write!(f, "Expecting an expression but got '{}'", tk.to_char())
            }
            ParseErrorKind::UnexpectedTokenAtEOF(tk) => {
                write!(f, "Expecting EOF but got '{}'", tk.to_char())
            }
            ParseErrorKind::Expecting(ex, got) => write!(
                f,
                "Expecting '{}' but got '{}'",
                ex.to_char(),
                got.to_char()
            ),
            ParseErrorKind::InvalidFunctionCall => {
                write!(f, "This is not a valid function call")
            }
            ParseErrorKind::ParseFloatError(err) => write!(f, "Parse float error: {err}"),
            ParseErrorKind::ParseIntError(err) => write!(f, "Parse int error: {err}"),
        }
    }
}
