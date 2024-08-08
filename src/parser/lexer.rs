
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
    Pipe,
    ExclamationMark,
    OpenParen,
    CloseParen,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub(crate) kind: TokenKind,
    pub(crate) value: LexValue<'a>,
}

#[derive(Debug)]
pub enum LexValue<'a> {
    None,
    Int(i64),
    Float(f64),
    Identifier(&'a [u8]),
}

pub fn lex<'a>(str: &'a [u8]) -> Result<Vec<Token<'a>>, String> {
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
            b'&' => TokenKind::Ampersand,
            b'|' => TokenKind::Pipe,
            b'^' => TokenKind::Caret,
            b'!' => TokenKind::ExclamationMark,
            b'.' => TokenKind::Period,
            b',' => TokenKind::Comma,
            b'(' => TokenKind::OpenParen,
            b')' => TokenKind::CloseParen,
            _ if c.is_ascii_whitespace() => {
                continue;
            }
            _ => panic!(),
        };

        match kind {
            TokenKind::Literal => {
                let number = &str[start_pos..pos];
                let number = core::str::from_utf8(number).unwrap();
                if number.contains('.') {
                    tokens.push(Token {
                        kind,
                        value: LexValue::Float(
                            str::parse(number)
                                .expect(&format!("expect float value got '{number}'")),
                        ),
                    });
                } else {
                    tokens.push(Token {
                        kind,
                        value: LexValue::Int(
                            str::parse(number).expect(&format!("expect int value got '{number}'")),
                        ),
                    });
                }
            }
            TokenKind::Identifier => {
                tokens.push(Token {
                    kind,
                    value: LexValue::Identifier(&str[start_pos..pos]),
                });
            }
            _ => {
                tokens.push(Token {
                    kind,
                    value: LexValue::None,
                });
            }
        }
    }

    Ok(tokens)
}
