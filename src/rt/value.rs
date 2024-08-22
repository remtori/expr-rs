#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Int(i64),
    Float(f64),
}

impl Value {
    pub fn as_float(&self) -> f64 {
        match self {
            Value::Int(v) => *v as f64,
            Value::Float(v) => *v,
        }
    }

    pub fn as_int(&self) -> i64 {
        match self {
            Value::Int(v) => *v,
            Value::Float(v) => v.floor() as i64,
        }
    }
}

impl Value {
    pub fn do_neg(v: Self) -> Self {
        match v {
            Value::Float(v) => Value::Float(-v),
            Value::Int(v) => Value::Int(-v),
        }
    }
}

macro_rules! binary_op {
    (math, $name:ident, $op:tt) => {
        pub fn $name(a: Self,  b: Self) -> Self {
            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a $op b),
                _ => {
                    let a = a.as_float();
                    let b = b.as_float();
                    Value::Float(a $op b)
                }
            }
        }
    };
    (logic, $name:ident, $op:tt) => {
        pub fn $name(a: Self,  b: Self) -> Self {
            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a $op b),
                _ => {
                    let a = a.as_int();
                    let b = b.as_int();
                    Value::Int(a $op b)
                }
            }
        }
    };
}

impl Value {
    binary_op!(math, do_add, +);
    binary_op!(math, do_sub, -);
    binary_op!(math, do_mul, *);
    binary_op!(math, do_div, /);
    binary_op!(math, do_mod, %);
    binary_op!(logic, do_and, &);
    binary_op!(logic, do_or, |);
    binary_op!(logic, do_xor, ^);
}
