use super::{error::ParseError, ParseErrorKind};
use crate::Span;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    Literal,
    Identifier,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Comma,
    Period,
    Caret,
    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,
    EqualEqual,
    ExclamationEqual,
    ExclamationMark,
    OpenParen,
    CloseParen,
}

impl TokenKind {
    pub fn to_char(&self) -> &'static str {
        match self {
            TokenKind::Literal => "<literal>",
            TokenKind::Identifier => "<ident>",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Asterisk => "*",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::Comma => ",",
            TokenKind::Period => ".",
            TokenKind::Caret => "^",
            TokenKind::Ampersand => "&",
            TokenKind::AmpersandAmpersand => "&&",
            TokenKind::Pipe => "|",
            TokenKind::PipePipe => "||",
            TokenKind::EqualEqual => "==",
            TokenKind::ExclamationEqual => "!=",
            TokenKind::ExclamationMark => "!",
            TokenKind::OpenParen => "(",
            TokenKind::CloseParen => ")",
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub(crate) kind: TokenKind,
    pub(crate) span: Span,
    pub(crate) value: LexValue<'a>,
}

#[derive(Debug)]
pub enum LexValue<'a> {
    None,
    Int(i64),
    Float(f64),
    Identifier(&'a [u8]),
}

pub fn lex<'a>(str: &'a [u8]) -> Result<Vec<Token<'a>>, ParseError> {
    let mut tokens: Vec<Token<'a>> = Vec::new();
    let mut pos = 0;
    while pos < str.len() {
        let start_pos = pos;
        let c = str[pos];
        pos += 1;

        let kind = match c {
            b'0'..=b'9' => {
                while pos < str.len() {
                    match str[pos] {
                        b'0'..=b'9' | b'.' => {
                            pos += 1;
                            continue;
                        }
                        _ => {
                            break;
                        }
                    }
                }

                TokenKind::Literal
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                while pos < str.len() {
                    if str[pos].is_ascii_alphanumeric() || str[pos] == b'_' {
                        pos += 1;
                        continue;
                    } else {
                        break;
                    }
                }

                TokenKind::Identifier
            }
            b'+' => TokenKind::Plus,
            b'-' => TokenKind::Minus,
            b'*' => TokenKind::Asterisk,
            b'/' => TokenKind::Slash,
            b'%' => TokenKind::Percent,
            b'^' => TokenKind::Caret,
            b'.' => TokenKind::Period,
            b',' => TokenKind::Comma,
            b'(' => TokenKind::OpenParen,
            b')' => TokenKind::CloseParen,
            b'=' => {
                if pos < str.len() && str[pos] == b'=' {
                    pos += 1;
                    TokenKind::EqualEqual
                } else {
                    return Err(ParseError::new(
                        ParseErrorKind::UnexpectedChar(c as char),
                        Span {
                            from: start_pos,
                            to: pos - 1,
                        },
                    ));
                }
            }
            b'!' => {
                if pos < str.len() && str[pos] == b'=' {
                    pos += 1;
                    TokenKind::ExclamationEqual
                } else {
                    TokenKind::ExclamationMark
                }
            }
            b'&' => {
                if pos < str.len() && str[pos] == b'&' {
                    pos += 1;
                    TokenKind::AmpersandAmpersand
                } else {
                    TokenKind::Ampersand
                }
            }
            b'|' => {
                if pos < str.len() && str[pos] == b'|' {
                    TokenKind::PipePipe
                } else {
                    TokenKind::Pipe
                }
            }
            _ if c.is_ascii_whitespace() => {
                continue;
            }
            _ => {
                return Err(ParseError::new(
                    ParseErrorKind::UnexpectedChar(c as char),
                    Span {
                        from: start_pos,
                        to: pos - 1,
                    },
                ));
            }
        };

        let span = Span {
            from: start_pos,
            to: pos - 1,
        };
        match kind {
            TokenKind::Literal => {
                let number = &str[start_pos..pos];
                let number = core::str::from_utf8(number).unwrap();
                if number.contains('.') {
                    tokens.push(Token {
                        kind,
                        span,
                        value: LexValue::Float(str::parse(number).map_err(|err| {
                            ParseError::new(ParseErrorKind::ParseFloatError(err), span)
                        })?),
                    });
                } else {
                    tokens.push(Token {
                        kind,
                        span,
                        value: LexValue::Int(str::parse(number).map_err(|err| {
                            ParseError::new(ParseErrorKind::ParseIntError(err), span)
                        })?),
                    });
                }
            }
            TokenKind::Identifier => {
                tokens.push(Token {
                    kind,
                    span,
                    value: LexValue::Identifier(&str[start_pos..pos]),
                });
            }
            _ => {
                tokens.push(Token {
                    kind,
                    span,
                    value: LexValue::None,
                });
            }
        }
    }

    Ok(tokens)
}
