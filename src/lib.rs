mod parser;
mod rt;
mod span;

use std::backtrace::{Backtrace, BacktraceStatus};

pub use parser::{Expr, ParseError, ParseErrorKind};
pub use rt::{IntoExtFunc, Program, Registry, RuntimeError, Value};
pub use span::Span;

pub fn eval_with_registry(registry: &mut Registry, source: &str) -> Result<Value, Error> {
    let expr = Expr::from_src(source.as_bytes())?;
    let program = Program::compile(registry, &expr)?;
    let result = program.run(registry)?;
    Ok(result)
}

pub fn eval(source: &str) -> Result<Value, Error> {
    let mut registry = Registry::default();
    eval_with_registry(&mut registry, source)
}

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    RuntimeError(RuntimeError),
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::ParseError(err)
    }
}

impl From<RuntimeError> for Error {
    fn from(err: RuntimeError) -> Self {
        Error::RuntimeError(err)
    }
}

impl Error {
    pub fn to_pretty_string(&self, source: &str) -> Result<String, std::io::Error> {
        use std::io::Write;

        let (err, span, backtrace): (&dyn std::fmt::Display, Option<Span>, Option<&Backtrace>) =
            match self {
                Error::ParseError(err) => (err, err.span(), Some(err.backtrace())),
                Error::RuntimeError(err) => (err, err.span(), None),
            };

        let out: &mut Vec<u8> = &mut Vec::with_capacity(source.len() + 1024);
        writeln!(out, "{source}")?;

        let mut offset = 0;
        if let Some(span) = span {
            offset = (span.from + span.to) / 2;
            for i in 0..=span.to {
                if i < span.from {
                    write!(out, " ")?;
                } else {
                    write!(out, "^")?;
                }
            }

            writeln!(out,)?;
        }

        writeln!(out, "{}{:#}", " ".repeat(offset), err)?;
        if let Some(backtrace) = backtrace {
            if backtrace.status() == BacktraceStatus::Captured {
                writeln!(out, "At\n{}", backtrace)?;
            }
        }

        Ok(String::from_utf8(std::mem::take(out)).expect("We only print valid utf8"))
    }
}
