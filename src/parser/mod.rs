mod error;
mod expr;
mod lexer;

pub use self::{
    error::{ParseError, ParseErrorKind},
    expr::{BinaryOp, Expr, UnaryOp},
};

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
            return Err(ParseError::new_nospan(ParseErrorKind::UnexpectedEOF));
        };

        let precedent = self.operator_precedent(tk.kind).unwrap_or(0);
        let expr = match tk.kind {
            TokenKind::Literal => {
                self.skip()?;
                match tk.value {
                    LexValue::Float(v) => Expr::Literal(v.into(), tk.span),
                    LexValue::Int(v) => Expr::Literal(v.into(), tk.span),
                    _ => unreachable!(),
                }
            }
            TokenKind::Identifier => {
                self.skip()?;
                match tk.value {
                    LexValue::Identifier(ident) => match ident {
                        b"true" => Expr::Literal(true.into(), tk.span),
                        b"false" => Expr::Literal(false.into(), tk.span),
                        ident => Expr::Identifier(Box::from(ident), tk.span),
                    },
                    _ => unreachable!(),
                }
            }
            TokenKind::OpenParen => {
                self.skip()?;
                let expr = self.parse_expr(precedent)?;
                self.consume(TokenKind::CloseParen)?;
                expr
            }
            TokenKind::ExclamationMark => {
                self.skip()?;
                let expr = self.parse_expr(precedent)?;
                Expr::UnaryOp(UnaryOp::Not, Box::new(expr), tk.span)
            }
            TokenKind::Minus => {
                self.skip()?;
                let expr = self.parse_expr(precedent)?;
                Expr::UnaryOp(UnaryOp::Neg, Box::new(expr), tk.span)
            }
            _ => {
                return Err(ParseError::new(
                    ParseErrorKind::UnexpectedPrimaryExpr(tk.kind),
                    tk.span,
                ))
            }
        };

        Ok(expr)
    }

    fn parse_secondary_expr(&mut self, lhs: Expr, min_precedent: i32) -> Result<Expr, ParseError> {
        let Some(tk) = self.peek() else {
            return Err(ParseError::new_nospan(ParseErrorKind::UnexpectedEOF));
        };

        Ok(match tk.kind {
            TokenKind::Plus => {
                self.skip()?;
                Expr::BinaryOp(
                    Box::new(lhs),
                    BinaryOp::Add,
                    Box::new(self.parse_expr(min_precedent)?),
                    tk.span,
                )
            }
            TokenKind::Minus => {
                self.skip()?;
                Expr::BinaryOp(
                    Box::new(lhs),
                    BinaryOp::Sub,
                    Box::new(self.parse_expr(min_precedent)?),
                    tk.span,
                )
            }
            TokenKind::Asterisk => {
                self.skip()?;
                Expr::BinaryOp(
                    Box::new(lhs),
                    BinaryOp::Mul,
                    Box::new(self.parse_expr(min_precedent)?),
                    tk.span,
                )
            }
            TokenKind::Slash => {
                self.skip()?;
                Expr::BinaryOp(
                    Box::new(lhs),
                    BinaryOp::Div,
                    Box::new(self.parse_expr(min_precedent)?),
                    tk.span,
                )
            }
            TokenKind::Percent => {
                self.skip()?;
                Expr::BinaryOp(
                    Box::new(lhs),
                    BinaryOp::Mod,
                    Box::new(self.parse_expr(min_precedent)?),
                    tk.span,
                )
            }
            TokenKind::Ampersand => {
                self.skip()?;
                Expr::BinaryOp(
                    Box::new(lhs),
                    BinaryOp::BitAnd,
                    Box::new(self.parse_expr(min_precedent)?),
                    tk.span,
                )
            }
            TokenKind::Pipe => {
                self.skip()?;
                Expr::BinaryOp(
                    Box::new(lhs),
                    BinaryOp::BitOr,
                    Box::new(self.parse_expr(min_precedent)?),
                    tk.span,
                )
            }
            TokenKind::Caret => {
                self.skip()?;
                Expr::BinaryOp(
                    Box::new(lhs),
                    BinaryOp::BitXor,
                    Box::new(self.parse_expr(min_precedent)?),
                    tk.span,
                )
            }
            TokenKind::AmpersandAmpersand => {
                self.skip()?;
                Expr::BinaryOp(
                    Box::new(lhs),
                    BinaryOp::LogicalAnd,
                    Box::new(self.parse_expr(min_precedent)?),
                    tk.span,
                )
            }
            TokenKind::PipePipe => {
                self.skip()?;
                Expr::BinaryOp(
                    Box::new(lhs),
                    BinaryOp::LogicalOr,
                    Box::new(self.parse_expr(min_precedent)?),
                    tk.span,
                )
            }
            TokenKind::EqualEqual => {
                self.skip()?;
                Expr::BinaryOp(
                    Box::new(lhs),
                    BinaryOp::Equal,
                    Box::new(self.parse_expr(min_precedent)?),
                    tk.span,
                )
            }
            TokenKind::ExclamationEqual => {
                self.skip()?;
                Expr::BinaryOp(
                    Box::new(lhs),
                    BinaryOp::NotEqual,
                    Box::new(self.parse_expr(min_precedent)?),
                    tk.span,
                )
            }
            TokenKind::OpenParen => {
                if let Expr::Identifier(ident, span) = lhs {
                    self.skip()?;
                    let mut args = Vec::new();
                    while let Some(tk) = self.peek() {
                        if tk.kind == TokenKind::CloseParen {
                            break;
                        }

                        args.push(self.parse_expr(0)?);
                        if let Some(tk) = self.peek() {
                            if tk.kind == TokenKind::Comma {
                                self.skip()?;
                                continue;
                            }

                            break;
                        }
                    }

                    self.consume(TokenKind::CloseParen)?;
                    Expr::Call(ident, args, span)
                } else {
                    return Err(ParseError::new(
                        ParseErrorKind::InvalidFunctionCall,
                        lhs.span(),
                    ));
                }
            }
            _ => unreachable!("{:?}", tk),
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
            _ => None,
        }
    }

    fn peek(&self) -> Option<&'a Token<'a>> {
        self.tokens.get(0)
    }

    fn consume(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if self.tokens.len() == 0 {
            return Err(ParseError::new_nospan(ParseErrorKind::ExpectingButGotEOF(
                kind,
            )));
        }

        if self.tokens[0].kind != kind {
            return Err(ParseError::new(
                ParseErrorKind::Expecting(kind, self.tokens[0].kind),
                self.tokens[0].span,
            ));
        }

        self.tokens = &self.tokens[1..];
        Ok(())
    }

    fn skip(&mut self) -> Result<(), ParseError> {
        if self.tokens.len() == 0 {
            return Err(ParseError::new_nospan(ParseErrorKind::UnexpectedEOF));
        }

        self.tokens = &self.tokens[1..];
        Ok(())
    }
}

impl Expr {
    pub fn from_src(source: &[u8]) -> Result<Expr, ParseError> {
        let tokens = lex(source)?;

        let mut parser = Parser { tokens: &tokens };
        let expr = parser.parse_expr(0)?;
        if let Some(tk) = parser.tokens.get(0) {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedTokenAtEOF(tk.kind),
                tk.span,
            ));
        }

        Ok(expr)
    }
}
