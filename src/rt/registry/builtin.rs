use crate::Value;

pub fn pow(a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Boolean(_), _) | (_, Value::Boolean(_)) => Value::Boolean(false),
        (Value::Int(a), Value::Int(b)) => Value::Int(a.pow(b as u32)),
        _ => {
            let a = a.to_float();
            let b = b.to_float();
            Value::Float(a.powf(b))
        }
    }
}

pub fn sin(a: Value) -> Value {
    match a {
        Value::Boolean(_) => Value::Boolean(false),
        Value::Int(a) => Value::Float((a as f64).sin()),
        Value::Float(a) => Value::Float(a.sin()),
    }
}
