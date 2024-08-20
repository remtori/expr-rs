use core::str;

use expr::{Expr, Program, Registry, Value};

fn main() {
    let registry = Registry {
        vars: vec![(b"z".to_vec(), Value::Int(99))],
        fns: vec![(b"pow".to_vec(), builtin::pow)],
    };

    let src = b"pow(3 * 93 * 10000 * 749, 2)";
    let expr = match Expr::from_src(src) {
        Ok(expr) => expr,
        Err(err) => {
            println!("{}", str::from_utf8(src).unwrap());

            let mut offset = 0;
            if let Some(span) = err.span() {
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

            #[cfg(feature = "backtrace")]
            println!("At\n{:?}", err.backtrace());
            return;
        }
    };

    let program = Program::compile(&registry, &expr).unwrap();
    println!("{:?}", program.run(&registry));
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
