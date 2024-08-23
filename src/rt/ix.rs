use crate::parser::{BinaryOp, Expr, UnaryOp};

use super::{RuntimeError, RuntimeErrorKind, Value};

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    #[allow(dead_code)]
    Noop,
    PushLit(Value),
    PushVariable {
        ident: u32,
    },
    Call {
        ident: u32,
        arg_count: u32,
    },
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
}

pub(crate) fn write_instruction(
    expr: &Expr,
    registry: &super::Registry,
    out: &mut Vec<Instruction>,
) -> Result<(), RuntimeError> {
    match expr {
        Expr::LitInt(v, _) => out.push(Instruction::PushLit(Value::Int(*v))),
        Expr::LitFloat(v, _) => out.push(Instruction::PushLit(Value::Float(*v))),
        Expr::Identifier(ident, span) => {
            let var = registry
                .vars
                .iter()
                .enumerate()
                .find(|(_, var)| &var.0 == ident);

            let (ident, _) = var.ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::UndeclaredVariable(
                        String::from_utf8_lossy(ident).to_string(),
                    ),
                    *span,
                )
            })?;

            out.push(Instruction::PushVariable {
                ident: ident as u32,
            });
        }
        Expr::Call(ident, args, span) => {
            for arg in args {
                write_instruction(arg, registry, out)?;
            }

            let func = registry
                .fns
                .iter()
                .enumerate()
                .find(|(_, func)| &func.0 == ident);

            let (ident, _) = func.ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::UndeclaredFunction(
                        String::from_utf8_lossy(ident).to_string(),
                    ),
                    *span,
                )
            })?;

            out.push(Instruction::Call {
                ident: ident as u32,
                arg_count: u32::try_from(args.len()).unwrap(),
            })
        }
        Expr::BinaryOp(a, op, b, _) => {
            write_instruction(a, registry, out)?;
            write_instruction(b, registry, out)?;
            out.push(Instruction::BinaryOp(*op));
        }
        Expr::UnaryOp(op, expr, _) => {
            write_instruction(expr, registry, out)?;
            out.push(Instruction::UnaryOp(*op));
        }
    };

    Ok(())
}
