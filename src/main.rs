mod parser;
mod rt;

fn main() {
    let registry = rt::Registry {
        vars: vec![(b"z".to_vec(), rt::Value::Int(99))],
        fns: vec![(b"pow".to_vec(), builtin::pow)],
    };

    let program = rt::Program::compile(&registry, b"1 + pow(2, 3) * 4").unwrap();
    println!("{:?}", program.run(&registry));
}

mod builtin {
    use crate::rt::Value;

    pub fn pow(args: &[Value]) -> Value {
        match (args[0], args[1]) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a.pow(b as u32)),
            _ => todo!(),
        }
    }
}
