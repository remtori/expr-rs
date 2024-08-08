use crate::parser::parse;

mod ix;

use ix::{Instruction, WriteInstruction};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Int(i64),
    Float(f64),
}

pub struct Registry {
    pub vars: Vec<(Vec<u8>, Value)>,
    pub fns: Vec<(Vec<u8>, fn(args: &[Value]) -> Value)>,
}

pub struct Program {
    instructions: Vec<ix::Instruction>,
}

impl Program {
    pub fn compile(registry: &Registry, source: &[u8]) -> Result<Program, String> {
        let tokens = parse(source)?;

        let mut instructions = Vec::new();
        tokens.write_instruction(registry, &mut instructions)?;
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
                    let args = &stack[stack.len() - arg_count as usize..stack.len()];
                    let ret = registry.fns[ident as usize].1(args);

                    for _ in 0..arg_count {
                        stack.pop();
                    }

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
                        _ => todo!(),
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

        stack.pop().unwrap()
    }
}
