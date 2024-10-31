use std::{
    backtrace::{Backtrace, BacktraceStatus},
    error::Error,
};

use expr::{Expr, Program, Registry, Span};

fn main() {
    let mut registry = Registry::new();
    registry.add_var(b"z", 99).add_fn(b"pow", builtin::pow);

    let src = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("pow(3 * 93 * 10000 * 749, 2)".to_string());

    let expr = match Expr::from_src(src.as_bytes()) {
        Ok(expr) => expr,
        Err(err) => {
            println!("{src}");
            print_span(&err, err.span(), Some(err.backtrace()));
            return;
        }
    };

    let program = match Program::compile(&registry, &expr) {
        Ok(p) => p,
        Err(err) => {
            println!("{src}");
            print_span(&err, err.span(), None);
            return;
        }
    };
    println!("{:#?}", program);
    println!("{:?}", program.run(&mut registry));
}

fn print_span(err: impl Error, span: Option<Span>, backtrace: Option<&Backtrace>) {
    let mut offset = 0;
    if let Some(span) = span {
        offset = (span.from + span.to) / 2;
        for i in 0..=span.to {
            if i < span.from {
                print!(" ");
            } else {
                print!("^");
            }
        }

        println!();
    }

    println!("{}{:#}", " ".repeat(offset), err);
    if let Some(backtrace) = backtrace {
        if backtrace.status() == BacktraceStatus::Captured {
            println!("At\n{}", backtrace);
        }
    }
}

mod builtin {
    use expr::Value;

    pub fn pow(a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a.pow(b as u32)),
            _ => todo!(),
        }
    }
}
