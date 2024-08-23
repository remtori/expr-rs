use crate::Span;

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    LogicalAnd,
    LogicalOr,
    BitAnd,
    BitOr,
    BitXor,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum Expr {
    LitInt(i64, Span),
    LitFloat(f64, Span),
    Identifier(Vec<u8>, Span),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>, Span),
    UnaryOp(UnaryOp, Box<Expr>, Span),
    Call(Vec<u8>, Vec<Expr>, Span),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::LitInt(_, s) => *s,
            Expr::LitFloat(_, s) => *s,
            Expr::Identifier(_, s) => *s,
            Expr::BinaryOp(_, _, _, s) => *s,
            Expr::UnaryOp(_, _, s) => *s,
            Expr::Call(_, _, s) => *s,
        }
    }
}
