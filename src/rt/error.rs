use std::{error::Error, fmt::Display};

use crate::Span;

#[derive(Debug)]
pub enum RuntimeErrorKind {
    UndeclaredVariable(String),
    UndeclaredFunction(String),
    WrongArgumentCount(usize, usize),
    MalformedInstructionStream,
}

#[derive(Debug)]
pub struct RuntimeError {
    kind: RuntimeErrorKind,
    span: Option<Span>,
}

impl RuntimeError {
    pub fn new(kind: RuntimeErrorKind, span: Span) -> Self {
        Self {
            kind,
            span: Some(span),
        }
    }

    pub fn span(&self) -> Option<Span> {
        self.span
    }
}

impl From<RuntimeErrorKind> for RuntimeError {
    fn from(kind: RuntimeErrorKind) -> Self {
        RuntimeError { kind, span: None }
    }
}

impl Error for RuntimeError {}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            RuntimeErrorKind::UndeclaredVariable(v) => write!(f, "Undeclared variable {}", v),
            RuntimeErrorKind::UndeclaredFunction(v) => write!(f, "Undeclared function {}", v),
            RuntimeErrorKind::WrongArgumentCount(expected, got) => write!(
                f,
                "Function called with wrong number of arguments (expected: {}, got: {})",
                expected, got
            ),
            RuntimeErrorKind::MalformedInstructionStream => {
                write!(f, "Malformed instruction stream")
            }
        }
    }
}
