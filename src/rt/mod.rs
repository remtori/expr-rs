use crate::parser::Expr;

mod error;
mod ix;
mod value;

use ix::Instruction;
pub use {error::RuntimeError, value::Value};

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
                Instruction::PushLit(v) => stack.push(v),
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
                    let ret = match ins {
                        Instruction::Add => Value::do_add(a, b),
                        Instruction::Sub => Value::do_sub(a, b),
                        Instruction::Mul => Value::do_mul(a, b),
                        Instruction::Div => Value::do_div(a, b),
                        Instruction::Mod => Value::do_mod(a, b),
                        Instruction::And => Value::do_and(a, b),
                        Instruction::Or => Value::do_or(a, b),
                        Instruction::Xor => Value::do_xor(a, b),
                        _ => unreachable!(),
                    };

                    stack.push(ret);
                }
                Instruction::Not => {
                    let v = stack.pop().unwrap();
                    let ret = Value::do_neg(v);
                    stack.push(ret);
                }
            }
        }

        debug_assert!(stack.len() == 1);
        stack.pop().unwrap()
    }
}
