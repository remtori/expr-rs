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
        Expr::Literal(v, _) => out.push(Instruction::PushLit(*v)),
        Expr::Identifier(ident, span) => {
            let var = registry.var_ident(ident);
            let ident = var.ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::UndeclaredVariable(
                        String::from_utf8_lossy(ident).to_string(),
                    ),
                    *span,
                )
            })?;

            out.push(Instruction::PushVariable { ident });
        }
        Expr::Call(ident, args, span) => {
            for arg in args {
                write_instruction(arg, registry, out)?;
            }

            let func = registry.fn_ident(ident);
            let (ident, arg_count) = func.ok_or_else(|| {
                RuntimeError::new(
                    RuntimeErrorKind::UndeclaredFunction(
                        String::from_utf8_lossy(ident).to_string(),
                    ),
                    *span,
                )
            })?;

            let supplied_arg_count = u32::try_from(args.len()).unwrap();
            if arg_count != u32::MAX && supplied_arg_count != arg_count {
                return Err(RuntimeError::new(
                    RuntimeErrorKind::WrongArgumentCount(arg_count, supplied_arg_count),
                    *span,
                ));
            }

            out.push(Instruction::Call {
                ident,
                arg_count: supplied_arg_count,
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
