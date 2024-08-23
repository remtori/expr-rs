use crate::parser::{Expr, UnaryOp};

mod error;
mod ix;
mod opt_pass;
mod value;

use self::{error::RuntimeErrorKind, ix::Instruction, opt_pass::run_optimize_pass};
pub use {error::RuntimeError, value::Value};

pub struct Registry {
    pub vars: Vec<(Vec<u8>, Value)>,
    pub fns: Vec<(Vec<u8>, fn(args: &[Value]) -> Value)>,
}

#[derive(Debug)]
pub struct Program {
    instructions: Vec<ix::Instruction>,
}

impl Program {
    pub fn compile(registry: &Registry, expr: &Expr) -> Result<Program, RuntimeError> {
        let mut instructions = Vec::new();
        ix::write_instruction(expr, registry, &mut instructions)?;

        let instructions = run_optimize_pass(instructions);
        Ok(Program { instructions })
    }

    pub fn run(&self, registry: &Registry) -> Result<Value, RuntimeError> {
        let mut stack = Vec::new();
        for ins in self.instructions.iter().copied() {
            match ins {
                Instruction::Noop => {}
                Instruction::PushLit(v) => stack.push(v),
                Instruction::PushVariable { ident } => stack.push(registry.vars[ident as usize].1),
                Instruction::Call { ident, arg_count } => {
                    let arg_count = arg_count as usize;
                    if arg_count > stack.len() {
                        return Err(RuntimeErrorKind::MalformedInstructionStream.into());
                    }

                    let args = &stack[stack.len() - arg_count..];
                    let ret = registry.fns[ident as usize].1(args);

                    stack.drain(stack.len() - arg_count..);
                    stack.push(ret);
                }
                Instruction::BinaryOp(op) => {
                    let b = stack
                        .pop()
                        .ok_or_else(|| RuntimeErrorKind::MalformedInstructionStream)?;
                    let a = stack
                        .pop()
                        .ok_or_else(|| RuntimeErrorKind::MalformedInstructionStream)?;

                    stack.push(Value::do_binary_op(a, b, op));
                }
                Instruction::UnaryOp(op) => {
                    let v = stack
                        .pop()
                        .ok_or_else(|| RuntimeErrorKind::MalformedInstructionStream)?;

                    let ret = match op {
                        UnaryOp::Neg => v.neg(),
                        UnaryOp::Not => v.not(),
                    };
                    stack.push(ret);
                }
            }
        }

        debug_assert!(stack.len() == 1);
        stack
            .pop()
            .ok_or_else(|| RuntimeErrorKind::MalformedInstructionStream.into())
    }
}
