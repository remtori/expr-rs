use expr::{Expr, Program, Registry, Value};

fn main() {
    let registry = Registry {
        vars: vec![(b"z".to_vec(), Value::Int(99))],
        fns: vec![(b"pow".to_vec(), builtin::pow)],
    };

    let expr = Expr::from_src(b"1 + pow(2, 3) * 4").unwrap();
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
