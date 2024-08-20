use crate::parser::Expr;

mod error;
mod ix;

pub use error::RuntimeError;
use ix::Instruction;

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
}

pub struct Registry {
    pub vars: Vec<(Vec<u8>, Value)>,
    pub fns: Vec<(Vec<u8>, fn(args: &[Value]) -> Value)>,
}

pub struct Program {
    instructions: Vec<ix::Instruction>,
}

impl Program {
    pub fn compile(registry: &Registry, expr: &Expr) -> Result<Program, RuntimeError> {
        let mut instructions = Vec::new();
        ix::write_instruction(expr, registry, &mut instructions)?;
        Ok(Program { instructions })
    }

    pub fn run(&self, registry: &Registry) -> Value {
        let mut stack = Vec::new();
        for ins in self.instructions.iter().copied() {
            match ins {
                Instruction::PushLitInt(v) => stack.push(Value::Int(v)),
                Instruction::PushLitFloat(v) => stack.push(Value::Float(v)),
                Instruction::PushVariable { ident } => stack.push(registry.vars[ident as usize].1),
                Instruction::Call { ident, arg_count } => {
                    debug_assert!(
                        (arg_count as usize) <= stack.len(),
                        "{} > {}",
                        arg_count,
                        stack.len()
                    );

                    let args = &stack[stack.len() - arg_count as usize..stack.len()];
                    let ret = registry.fns[ident as usize].1(args);

                    stack.drain(stack.len() - arg_count as usize..stack.len());
                    stack.push(ret);
                }
                Instruction::Add
                | Instruction::Sub
                | Instruction::Mul
                | Instruction::Div
                | Instruction::Mod
                | Instruction::And
                | Instruction::Or
                | Instruction::Xor => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let ret = match (a, b) {
                        (Value::Int(a), Value::Int(b)) => match ins {
                            Instruction::Add => Value::Int(a + b),
                            Instruction::Sub => Value::Int(a - b),
                            Instruction::Mul => Value::Int(a * b),
                            Instruction::Div => Value::Int(a / b),
                            Instruction::Mod => Value::Int(a % b),
                            Instruction::And => Value::Int(a & b),
                            Instruction::Or => Value::Int(a | b),
                            Instruction::Xor => Value::Int(a ^ b),
                            _ => unreachable!(),
                        },
                        _ => {
                            let a = a.as_float();
                            let b = b.as_float();
                            match ins {
                                Instruction::Add => Value::Float(a + b),
                                Instruction::Sub => Value::Float(a - b),
                                Instruction::Mul => Value::Float(a * b),
                                Instruction::Div => Value::Float(a / b),
                                Instruction::Mod => Value::Float(a % b),
                                _ => unreachable!(),
                            }
                        }
                    };

                    stack.push(ret);
                }
                Instruction::Not => {
                    let v = stack.pop().unwrap();
                    let ret = match v {
                        Value::Float(v) => Value::Float(-v),
                        Value::Int(v) => Value::Int(-v),
                    };

                    stack.push(ret);
                }
            }
        }

        debug_assert!(stack.len() == 1);
        stack.pop().unwrap()
    }
}
