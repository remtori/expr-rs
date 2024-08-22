use crate::parser::Expr;

use super::{RuntimeError, Value};

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Noop,
    PushLit(Value),
    PushVariable { ident: u32 },
    Call { ident: u32, arg_count: u32 },
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Not,
}

pub(crate) fn write_instruction(
    expr: &Expr,
    registry: &super::Registry,
    out: &mut Vec<Instruction>,
) -> Result<(), RuntimeError> {
    match expr {
        Expr::LitInt(v, _) => out.push(Instruction::PushLit(Value::Int(*v))),
        Expr::LitFloat(v, _) => out.push(Instruction::PushLit(Value::Float(*v))),
        Expr::Identifier(ident, _) => {
            let var = registry
                .vars
                .iter()
                .enumerate()
                .find(|(_, var)| &var.0 == ident);

            let (ident, _) = var.ok_or_else(|| {
                RuntimeError::UndeclaredVariable(String::from_utf8_lossy(ident).to_string())
            })?;

            out.push(Instruction::PushVariable {
                ident: ident as u32,
            });
        }
        Expr::Call(ident, args, _) => {
            for arg in args {
                write_instruction(arg, registry, out)?;
            }

            let func = registry
                .fns
                .iter()
                .enumerate()
                .find(|(_, func)| &func.0 == ident);

            let (ident, _) = func.ok_or_else(|| {
                RuntimeError::UndeclaredFunction(String::from_utf8_lossy(ident).to_string())
            })?;

            out.push(Instruction::Call {
                ident: ident as u32,
                arg_count: u32::try_from(args.len()).map_err(|_| RuntimeError::TooManyArguments)?,
            })
        }
        Expr::Add(a, b, _)
        | Expr::Sub(a, b, _)
        | Expr::Mul(a, b, _)
        | Expr::Div(a, b, _)
        | Expr::Mod(a, b, _)
        | Expr::And(a, b, _)
        | Expr::Or(a, b, _)
        | Expr::Xor(a, b, _) => {
            write_instruction(a, registry, out)?;
            write_instruction(b, registry, out)?;
            match expr {
                Expr::Add(_, _, _) => out.push(Instruction::Add),
                Expr::Sub(_, _, _) => out.push(Instruction::Sub),
                Expr::Mul(_, _, _) => out.push(Instruction::Mul),
                Expr::Div(_, _, _) => out.push(Instruction::Div),
                Expr::Mod(_, _, _) => out.push(Instruction::Mod),
                Expr::And(_, _, _) => out.push(Instruction::And),
                Expr::Or(_, _, _) => out.push(Instruction::Or),
                Expr::Xor(_, _, _) => out.push(Instruction::Xor),
                _ => unreachable!(),
            }
        }
        Expr::Not(expr, _) => {
            write_instruction(expr, registry, out)?;
            out.push(Instruction::Not);
        }
    };

    Ok(())
}
