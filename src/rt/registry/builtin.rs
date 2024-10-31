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

macro_rules! impl_float_fn {
    ($func:ident) => {
        pub fn $func(a: Value) -> Value {
            match a {
                Value::Boolean(_) => Value::Boolean(false),
                Value::Int(a) => Value::Float((a as f64).$func()),
                Value::Float(a) => Value::Float(a.$func()),
            }
        }
    };
}

impl_float_fn!(sin);
impl_float_fn!(cos);
impl_float_fn!(tan);
impl_float_fn!(asin);
impl_float_fn!(acos);
impl_float_fn!(atan);
impl_float_fn!(sinh);
impl_float_fn!(cosh);
impl_float_fn!(tanh);
impl_float_fn!(asinh);
impl_float_fn!(acosh);
impl_float_fn!(atanh);
impl_float_fn!(exp);
impl_float_fn!(ln);
impl_float_fn!(log10);
impl_float_fn!(log2);
impl_float_fn!(sqrt);
impl_float_fn!(cbrt);

pub fn sum(args: &[Value]) -> Value {
    let mut sum = 0.0;
    for arg in args {
        match arg {
            Value::Boolean(b) => sum += if *b { 1.0 } else { 0.0 },
            Value::Int(i) => sum += *i as f64,
            Value::Float(f) => sum += *f,
        }
    }

    Value::Float(sum)
}

pub fn max(args: &[Value]) -> Value {
    let mut max = f64::MIN;
    for arg in args {
        match arg {
            Value::Boolean(b) => max = if *b { 1.0 } else { 0.0 },
            Value::Int(i) => max = *i as f64,
            Value::Float(f) => max = *f,
        }
    }

    Value::Float(max)
}

pub fn min(args: &[Value]) -> Value {
    let mut min = f64::MAX;
    for arg in args {
        match arg {
            Value::Boolean(b) => min = if *b { 1.0 } else { 0.0 },
            Value::Int(i) => min = *i as f64,
            Value::Float(f) => min = *f,
        }
    }

    Value::Float(min)
}
