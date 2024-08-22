use super::{ix::Instruction, Value};

pub(crate) fn run_optimize_pass(mut ix_stream: Vec<Instruction>) -> Vec<Instruction> {
    for _ in 0..10 {
        constant_folding(&mut ix_stream);
        ix_stream.retain(|ix| !matches!(ix, Instruction::Noop));
    }

    ix_stream
}

fn windows_mut_each<const N: usize, T>(v: &mut [T], mut f: impl FnMut(&mut [T; N])) {
    let mut start = 0;
    let mut end = N;
    while end <= v.len() {
        f((&mut v[start..end]).try_into().unwrap());
        start += 1;
        end += 1;
    }
}

fn constant_folding(ix_stream: &mut Vec<Instruction>) {
    let ix_stream: &mut [Instruction] = &mut ix_stream[..];
    // let ix_stream = Cell::from_mut(ix_stream).as_slice_of_cells();
    windows_mut_each(ix_stream, |w: &mut [Instruction; 3]| match w {
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

            *w = [
                Instruction::PushLit(ret),
                Instruction::Noop,
                Instruction::Noop,
            ];
        }
        _ => {}
    });
}
