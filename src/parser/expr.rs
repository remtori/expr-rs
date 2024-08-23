use crate::Span;

#[derive(Debug)]
pub enum Expr {
    LitInt(i64, Span),
    LitFloat(f64, Span),
    Identifier(Vec<u8>, Span),
    Add(Box<Expr>, Box<Expr>, Span),
    Sub(Box<Expr>, Box<Expr>, Span),
    Mul(Box<Expr>, Box<Expr>, Span),
    Div(Box<Expr>, Box<Expr>, Span),
    Mod(Box<Expr>, Box<Expr>, Span),
    And(Box<Expr>, Box<Expr>, Span),
    Or(Box<Expr>, Box<Expr>, Span),
    Xor(Box<Expr>, Box<Expr>, Span),
    Not(Box<Expr>, Span),
    Call(Vec<u8>, Vec<Expr>, Span),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::LitInt(_, s) => *s,
            Expr::LitFloat(_, s) => *s,
            Expr::Identifier(_, s) => *s,
            Expr::Add(_, _, s) => *s,
            Expr::Sub(_, _, s) => *s,
            Expr::Mul(_, _, s) => *s,
            Expr::Div(_, _, s) => *s,
            Expr::Mod(_, _, s) => *s,
            Expr::And(_, _, s) => *s,
            Expr::Or(_, _, s) => *s,
            Expr::Xor(_, _, s) => *s,
            Expr::Not(_, s) => *s,
            Expr::Call(_, _, s) => *s,
        }
    }
}
