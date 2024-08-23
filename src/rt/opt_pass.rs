use super::{ix::Instruction, Value};

pub(crate) fn run_optimize_pass(mut ix_stream: Vec<Instruction>) -> Vec<Instruction> {
    for _ in 0..3 {
        constant_folding(&mut ix_stream);
        ix_stream.retain(|ix| !matches!(ix, Instruction::Noop));
    }

    ix_stream
}

fn constant_folding(ix_stream: &mut Vec<Instruction>) {
    for i in 0..ix_stream.len() {
        if i + 3 < ix_stream.len() {
            match &ix_stream[i..i + 3] {
                [Instruction::PushLit(a), Instruction::PushLit(b), op] => {
                    let a = *a;
                    let b = *b;
                    let ret = match op {
                        Instruction::Add => Value::do_add(a, b),
                        Instruction::Sub => Value::do_sub(a, b),
                        Instruction::Mul => Value::do_mul(a, b),
                        Instruction::Div => Value::do_div(a, b),
                        Instruction::Mod => Value::do_mod(a, b),
                        Instruction::And => Value::do_and(a, b),
                        Instruction::Or => Value::do_or(a, b),
                        Instruction::Xor => Value::do_xor(a, b),
                        _ => return,
                    };

                    ix_stream[i] = Instruction::PushLit(ret);
                    ix_stream.drain(i + 1..i + 3);
                }
                _ => {}
            }
        }

        if i + 2 < ix_stream.len() {
            match &ix_stream[i..i + 2] {
                [Instruction::PushLit(lit), Instruction::Not] => {
                    ix_stream[i] = Instruction::PushLit(Value::do_neg(*lit));
                    ix_stream.remove(i + 1);
                }
                _ => {}
            }
        }
    }
}
