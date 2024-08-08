use thiserror::Error;

use super::lexer::TokenKind;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    #[error("Unexpected character {0}")]
    UnexpectedChar(char),
    #[error("Cannot start an expression with {0:?}")]
    UnexpectedPrimaryExpr(TokenKind),
    #[error("Expecting {0:?} got {1:?}")]
    Expecting(TokenKind, TokenKind),
    #[error("Expecting {0:?} got EOF")]
    ExpectingButGotEOF(TokenKind),
    #[error("Invalid function call")]
    InvalidFunctionCall,
    #[error("Invalid value")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Invalid value")]
    ParseIntError(#[from] std::num::ParseIntError),
}
