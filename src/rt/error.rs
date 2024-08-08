use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Undeclared variable {0}")]
    UndeclaredVariable(String),
    #[error("Undeclared function {0}")]
    UndeclaredFunction(String),
    #[error("Function called with too many arguments")]
    TooManyArguments,
    #[error("Function expect {0} arguments but got {1}")]
    WrongArgumentCount(usize, usize),
    #[error("Malformed instruction stream")]
    MalformedInstructionStream,
}
