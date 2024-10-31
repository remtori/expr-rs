use crate::parser::BinaryOp;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Int(i64),
    Float(f64),
    Boolean(bool),
}

impl Value {
    pub fn to_float(&self) -> f64 {
        match self {
            Value::Int(v) => *v as f64,
            Value::Float(v) => *v,
            Value::Boolean(v) => {
                if *v {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }

    pub fn to_int(&self) -> i64 {
        match self {
            Value::Int(v) => *v,
            Value::Float(v) => v.floor() as i64,
            Value::Boolean(v) => {
                if *v {
                    1
                } else {
                    0
                }
            }
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Value::Int(v) => *v != 0,
            Value::Float(v) => !v.is_nan() && *v != 0.0,
            Value::Boolean(v) => *v,
        }
    }

    pub fn neg(&self) -> Self {
        match self {
            Value::Float(v) => Value::Float(-v),
            Value::Int(v) => Value::Int(-v),
            Value::Boolean(v) => Value::Int(if *v { -1 } else { 0 }),
        }
    }

    pub fn not(&self) -> Self {
        Value::Int(!self.to_int())
    }

    pub fn equals(a: Self, b: Self) -> bool {
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Int(b), Value::Float(a)) | (Value::Float(a), Value::Int(b)) => {
                a.trunc() == a && a as i64 == b
            }
            (Value::Int(a), Value::Boolean(b)) | (Value::Boolean(b), Value::Int(a)) => {
                a == if b { 1 } else { 0 }
            }
            (Value::Float(a), Value::Boolean(b)) | (Value::Boolean(b), Value::Float(a)) => {
                a == if b { 1.0 } else { 0.0 }
            }
        }
    }

    pub fn do_binary_op(a: Self, b: Self, op: BinaryOp) -> Self {
        match op {
            BinaryOp::Add => Value::do_add(a, b),
            BinaryOp::Sub => Value::do_sub(a, b),
            BinaryOp::Mul => Value::do_mul(a, b),
            BinaryOp::Div => Value::do_div(a, b),
            BinaryOp::Mod => Value::do_mod(a, b),
            BinaryOp::Equal => Value::equals(a, b).into(),
            BinaryOp::NotEqual => (!Value::equals(a, b)).into(),
            BinaryOp::BitAnd => Value::do_bitwise_and(a, b),
            BinaryOp::BitOr => Value::do_bitwise_or(a, b),
            BinaryOp::BitXor => Value::do_bitwise_xor(a, b),
            BinaryOp::LogicalAnd => Value::do_logical_and(a, b),
            BinaryOp::LogicalOr => Value::do_logical_or(a, b),
        }
    }
}

macro_rules! binary_op {
    (math, $name:ident, $op:tt) => {
        pub fn $name(a: Self,  b: Self) -> Self {
            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a $op b),
                _ => {
                    let a = a.to_float();
                    let b = b.to_float();
                    Value::Float(a $op b)
                }
            }
        }
    };
    (bitwise, $name:ident, $op:tt) => {
        pub fn $name(a: Self,  b: Self) -> Self {
            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a $op b),
                _ => {
                    let a = a.to_int();
                    let b = b.to_int();
                    Value::Int(a $op b)
                }
            }
        }
    };
    (logical, $name:ident, $op:tt) => {
        pub fn $name(a: Self,  b: Self) -> Self {
            let a = a.to_bool();
            let b = b.to_bool();

            Value::Boolean(a $op b)
        }
    };
}

impl Value {
    binary_op!(math, do_add, +);
    binary_op!(math, do_sub, -);
    binary_op!(math, do_mul, *);
    binary_op!(math, do_div, /);
    binary_op!(math, do_mod, %);
    binary_op!(bitwise, do_bitwise_and, &);
    binary_op!(bitwise, do_bitwise_or, |);
    binary_op!(bitwise, do_bitwise_xor, ^);
    binary_op!(logical, do_logical_and, &&);
    binary_op!(logical, do_logical_or, ||);
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Boolean(v) => write!(f, "{}", v),
        }
    }
}
