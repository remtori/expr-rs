use std::{
    backtrace::{Backtrace, BacktraceStatus},
    error::Error,
};

use expr::{Expr, Program, Registry, Span, Value};

fn main() {
    let registry = Registry {
        vars: vec![(b"z".to_vec(), Value::Int(99))],
        fns: vec![(b"pow".to_vec(), builtin::pow)],
    };

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
    println!("{:?}", program.run(&registry));
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

    pub fn pow(args: &[Value]) -> Value {
        match (args[0], args[1]) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a.pow(b as u32)),
            _ => todo!(),
        }
    }
}
