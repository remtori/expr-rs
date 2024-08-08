mod error;
mod expr;
mod lexer;

pub use self::{error::ParseError, expr::Expr};

use self::lexer::{lex, LexValue, Token, TokenKind};

struct Parser<'a> {
    tokens: &'a [Token<'a>],
}

impl<'a> Parser<'a> {
    fn parse_expr(&mut self, min_precedent: i32) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary_expr()?;
        while let Some(tk) = self.peek() {
            let Some(precedent) = self.operator_precedent(tk.kind) else {
                break;
            };

            if precedent <= min_precedent {
                break;
            }

            expr = self.parse_secondary_expr(expr, precedent)?;
        }

        Ok(expr)
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, ParseError> {
        let Some(tk) = self.peek() else {
            return Err(ParseError::UnexpectedEOF);
        };

        let expr = match tk.kind {
            TokenKind::Literal => {
                self.skip()?;
                match tk.value {
                    LexValue::Float(v) => Expr::LitFloat(v),
                    LexValue::Int(v) => Expr::LitInt(v),
                    _ => unreachable!(),
                }
            }
            TokenKind::Identifier => {
                self.skip()?;
                match tk.value {
                    LexValue::Identifier(ident) => Expr::Identifier(ident.to_vec()),
                    _ => unreachable!(),
                }
            }
            TokenKind::OpenParen => {
                self.skip()?;
                let expr = self.parse_expr(0)?;
                self.consume(TokenKind::CloseParen)?;
                expr
            }
            TokenKind::ExclamationMark => {
                self.skip()?;
                let expr = self.parse_expr(0)?;
                Expr::Not(Box::new(expr))
            }
            _ => return Err(ParseError::UnexpectedPrimaryExpr(tk.kind)),
        };

        Ok(expr)
    }

    fn parse_secondary_expr(&mut self, lhs: Expr, min_precedent: i32) -> Result<Expr, ParseError> {
        let Some(tk) = self.peek() else {
            return Err(ParseError::UnexpectedEOF);
        };

        Ok(match tk.kind {
            TokenKind::Plus => {
                self.skip()?;
                Expr::Add(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Minus => {
                self.skip()?;
                Expr::Sub(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Asterisk => {
                self.skip()?;
                Expr::Mul(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Slash => {
                self.skip()?;
                Expr::Div(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Percent => {
                self.skip()?;
                Expr::Mod(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Ampersand => {
                self.skip()?;
                Expr::And(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Pipe => {
                self.skip()?;
                Expr::Or(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Caret => {
                self.skip()?;
                Expr::Xor(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::OpenParen => {
                if let Expr::Identifier(ident) = lhs {
                    self.skip()?;
                    let mut args = Vec::new();
                    while let Some(tk) = self.peek() {
                        if tk.kind == TokenKind::CloseParen {
                            break;
                        }

                        args.push(self.parse_expr(min_precedent)?);
                        if let Some(tk) = self.peek() {
                            if tk.kind == TokenKind::Comma {
                                self.skip()?;
                                continue;
                            }

                            break;
                        }
                    }

                    self.consume(TokenKind::CloseParen)?;
                    Expr::Call(ident, args)
                } else {
                    return Err(ParseError::InvalidFunctionCall);
                }
            }
            _ => unreachable!(),
        })
    }

    fn operator_precedent(&self, kind: TokenKind) -> Option<i32> {
        match kind {
            TokenKind::OpenParen => Some(20),
            TokenKind::ExclamationMark => Some(17),
            TokenKind::Asterisk | TokenKind::Percent | TokenKind::Slash => Some(12),
            TokenKind::Plus | TokenKind::Minus => Some(11),
            TokenKind::Ampersand => Some(7),
            TokenKind::Caret => Some(6),
            TokenKind::Pipe => Some(5),
            TokenKind::Comma => Some(1),
            _ => None,
        }
    }

    fn peek(&self) -> Option<&'a Token<'a>> {
        self.tokens.get(0)
    }

    fn consume(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if self.tokens.len() == 0 {
            return Err(ParseError::ExpectingButGotEOF(kind));
        }

        if self.tokens[0].kind != kind {
            return Err(ParseError::Expecting(kind, self.tokens[0].kind));
        }

        self.tokens = &self.tokens[1..];
        Ok(())
    }

    fn skip(&mut self) -> Result<(), ParseError> {
        if self.tokens.len() == 0 {
            return Err(ParseError::UnexpectedEOF);
        }

        self.tokens = &self.tokens[1..];
        Ok(())
    }
}

impl Expr {
    pub fn from_src(source: &[u8]) -> Result<Expr, ParseError> {
        let tokens = lex(source)?;
        Parser { tokens: &tokens }.parse_expr(0)
    }
}
