use crate::parser::Expr;

use super::RuntimeError;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    PushLitInt(i64),
    PushLitFloat(f64),
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

pub(crate) trait WriteInstruction {
    fn write_instruction(
        &self,
        registry: &super::Registry,
        out: &mut Vec<Instruction>,
    ) -> Result<(), RuntimeError>;
}

impl WriteInstruction for Expr {
    fn write_instruction(
        &self,
        registry: &super::Registry,
        out: &mut Vec<Instruction>,
    ) -> Result<(), RuntimeError> {
        match self {
            Expr::LitInt(v) => out.push(Instruction::PushLitInt(*v)),
            Expr::LitFloat(v) => out.push(Instruction::PushLitFloat(*v)),
            Expr::Identifier(ident) => {
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
            Expr::Call(ident, args) => {
                for arg in args {
                    arg.write_instruction(registry, out)?;
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
                    arg_count: u32::try_from(args.len())
                        .map_err(|_| RuntimeError::TooManyArguments)?,
                })
            }
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Mod(a, b)
            | Expr::And(a, b)
            | Expr::Or(a, b)
            | Expr::Xor(a, b) => {
                a.write_instruction(registry, out)?;
                b.write_instruction(registry, out)?;
                match self {
                    Expr::Add(_, _) => out.push(Instruction::Add),
                    Expr::Sub(_, _) => out.push(Instruction::Sub),
                    Expr::Mul(_, _) => out.push(Instruction::Mul),
                    Expr::Div(_, _) => out.push(Instruction::Div),
                    Expr::Mod(_, _) => out.push(Instruction::Mod),
                    Expr::And(_, _) => out.push(Instruction::And),
                    Expr::Or(_, _) => out.push(Instruction::Or),
                    Expr::Xor(_, _) => out.push(Instruction::Xor),
                    _ => unreachable!(),
                }
            }
            Expr::Not(expr) => {
                expr.write_instruction(registry, out)?;
                out.push(Instruction::Not);
            }
        };

        Ok(())
    }
}
