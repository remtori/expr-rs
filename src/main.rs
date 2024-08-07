#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TokenKind {
    Literal,
    Identifier,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Comma,
    Period,
    Caret,
    Ampersand,
    Pipe,
    ExclamationMark,
    OpenParen,
    CloseParen,
}

#[derive(Debug)]
struct Token<'a> {
    kind: TokenKind,
    value: LexValue<'a>,
}

#[derive(Debug)]
enum LexValue<'a> {
    None,
    Int(i64),
    Float(f64),
    Identifer(&'a [u8]),
}

fn lex<'a>(str: &'a [u8]) -> Result<Vec<Token<'a>>, String> {
    let mut tokens: Vec<Token<'a>> = Vec::new();
    let mut pos = 0;
    while pos < str.len() {
        let start_pos = pos;
        let c = str[pos];
        pos += 1;

        let kind = match c {
            b'0'..=b'9' => {
                while pos < str.len() {
                    match str[pos] {
                        b'0'..=b'9' | b'.' => {
                            pos += 1;
                            continue;
                        }
                        _ => {
                            break;
                        }
                    }
                }

                TokenKind::Literal
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                while pos < str.len() {
                    if str[pos].is_ascii_alphanumeric() || str[pos] == b'_' {
                        pos += 1;
                        continue;
                    } else {
                        break;
                    }
                }

                TokenKind::Identifier
            }
            b'+' => TokenKind::Plus,
            b'-' => TokenKind::Minus,
            b'*' => TokenKind::Asterisk,
            b'/' => TokenKind::Slash,
            b'%' => TokenKind::Percent,
            b'&' => TokenKind::Ampersand,
            b'|' => TokenKind::Pipe,
            b'^' => TokenKind::Caret,
            b'!' => TokenKind::ExclamationMark,
            b'.' => TokenKind::Period,
            b',' => TokenKind::Comma,
            b'(' => TokenKind::OpenParen,
            b')' => TokenKind::CloseParen,
            _ if c.is_ascii_whitespace() => {
                continue;
            }
            _ => panic!(),
        };

        match kind {
            TokenKind::Literal => {
                let number = &str[start_pos..pos];
                let number = core::str::from_utf8(number).unwrap();
                if number.contains('.') {
                    tokens.push(Token {
                        kind,
                        value: LexValue::Float(
                            str::parse(number)
                                .expect(&format!("expect float value got '{number}'")),
                        ),
                    });
                } else {
                    tokens.push(Token {
                        kind,
                        value: LexValue::Int(
                            str::parse(number).expect(&format!("expect int value got '{number}'")),
                        ),
                    });
                }
            }
            TokenKind::Identifier => {
                tokens.push(Token {
                    kind,
                    value: LexValue::Identifer(&str[start_pos..pos]),
                });
            }
            _ => {
                tokens.push(Token {
                    kind,
                    value: LexValue::None,
                });
            }
        }
    }

    Ok(tokens)
}

#[derive(Debug)]
enum Expr {
    LitInt(i64),
    LitFloat(f64),
    Identifer(Vec<u8>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Call(Vec<u8>, Vec<Expr>),
}

struct Parser<'a> {
    tokens: &'a [Token<'a>],
}

impl<'a> Parser<'a> {
    fn parse_expr(&mut self, min_precedent: i32) -> Result<Expr, String> {
        let mut expr = self.parse_primary_expr()?;
        while let Some(tk) = self.peek() {
            let Some(precedent) = self.operator_precedent(tk.kind) else {
                break;
            };

            if precedent <= min_precedent {
                break;
            }

            expr = self.parse_secondary_expr(expr, precedent)?;
        }

        Ok(expr)
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, String> {
        let Some(tk) = self.peek() else {
            return Err("Unexpected EOF".to_owned());
        };

        let expr = match tk.kind {
            TokenKind::Literal => {
                self.skip()?;
                match tk.value {
                    LexValue::Float(v) => Expr::LitFloat(v),
                    LexValue::Int(v) => Expr::LitInt(v),
                    _ => unreachable!(),
                }
            }
            TokenKind::Identifier => {
                self.skip()?;
                match tk.value {
                    LexValue::Identifer(ident) => Expr::Identifer(ident.to_vec()),
                    _ => unreachable!(),
                }
            }
            TokenKind::OpenParen => {
                self.skip()?;
                let expr = self.parse_expr(0)?;
                self.consume(TokenKind::CloseParen)?;
                expr
            }
            TokenKind::ExclamationMark => {
                self.skip()?;
                let expr = self.parse_expr(0)?;
                Expr::Not(Box::new(expr))
            }
            _ => unreachable!(),
        };

        Ok(expr)
    }

    fn parse_secondary_expr(&mut self, lhs: Expr, min_precedent: i32) -> Result<Expr, String> {
        let Some(tk) = self.peek() else {
            return Err("unexpected EOF".to_owned());
        };

        Ok(match tk.kind {
            TokenKind::Plus => {
                self.skip()?;
                Expr::Add(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Minus => {
                self.skip()?;
                Expr::Sub(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Asterisk => {
                self.skip()?;
                Expr::Mul(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Slash => {
                self.skip()?;
                Expr::Div(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Percent => {
                self.skip()?;
                Expr::Mod(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Ampersand => {
                self.skip()?;
                Expr::And(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Pipe => {
                self.skip()?;
                Expr::Or(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::Caret => {
                self.skip()?;
                Expr::Xor(Box::new(lhs), Box::new(self.parse_expr(min_precedent)?))
            }
            TokenKind::OpenParen => {
                if let Expr::Identifer(ident) = lhs {
                    self.skip()?;
                    let mut args = Vec::new();
                    while let Some(tk) = self.peek() {
                        if tk.kind == TokenKind::CloseParen {
                            break;
                        }

                        args.push(self.parse_expr(min_precedent)?);
                        if let Some(tk) = self.peek() {
                            if tk.kind == TokenKind::Comma {
                                self.skip()?;
                                continue;
                            }

                            break;
                        }
                    }

                    self.consume(TokenKind::CloseParen)?;
                    Expr::Call(ident, args)
                } else {
                    return Err(format!("Cannot call {:?} as a function", lhs));
                }
            }
            _ => unreachable!(),
        })
    }

    fn operator_precedent(&self, kind: TokenKind) -> Option<i32> {
        match kind {
            TokenKind::OpenParen => Some(20),
            TokenKind::ExclamationMark => Some(17),
            TokenKind::Asterisk | TokenKind::Percent | TokenKind::Slash => Some(12),
            TokenKind::Plus | TokenKind::Minus => Some(11),
            TokenKind::Ampersand => Some(7),
            TokenKind::Caret => Some(6),
            TokenKind::Pipe => Some(5),
            TokenKind::Comma => Some(1),
            _ => None,
        }
    }

    fn peek(&self) -> Option<&'a Token<'a>> {
        self.tokens.get(0)
    }

    fn consume(&mut self, kind: TokenKind) -> Result<(), String> {
        if self.tokens.len() == 0 {
            return Err(format!("Expected {kind:?} got EOF"));
        }

        if self.tokens[0].kind != kind {
            return Err(format!("Expected {kind:?} got {:?}", self.tokens[0].kind));
        }

        self.tokens = &self.tokens[1..];
        Ok(())
    }

    fn skip(&mut self) -> Result<(), String> {
        if self.tokens.len() == 0 {
            return Err(format!("Unexpected EOF"));
        }

        self.tokens = &self.tokens[1..];
        Ok(())
    }
}

fn parse(tokens: &[Token<'_>]) -> Result<Expr, String> {
    Parser { tokens }.parse_expr(0)
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Value {
    Int(i64),
    Float(f64),
}

struct Registry {
    vars: Vec<(Vec<u8>, Value)>,
    fns: Vec<(Vec<u8>, fn(args: &[Value]) -> Value)>,
}

impl Expr {
    fn write_instructions(&self, registry: &Registry, out: &mut Vec<Instruction>) {
        match self {
            Expr::LitInt(v) => out.push(Instruction::PushLitInt(*v)),
            Expr::LitFloat(v) => out.push(Instruction::PushLitFloat(*v)),
            Expr::Identifer(ident) => {
                let var = registry
                    .vars
                    .iter()
                    .enumerate()
                    .find(|(_, var)| &var.0 == ident);

                let (ident, _) = var.expect(&format!(
                    "undeclared variable {}",
                    String::from_utf8_lossy(ident)
                ));

                out.push(Instruction::PushVariable {
                    ident: ident as u32,
                });
            }
            Expr::Call(ident, args) => {
                args.iter()
                    .for_each(|arg| arg.write_instructions(registry, out));

                let func = registry
                    .fns
                    .iter()
                    .enumerate()
                    .find(|(_, func)| &func.0 == ident);

                let (ident, _) = func.expect(&format!(
                    "undeclared function {}",
                    String::from_utf8_lossy(ident)
                ));

                out.push(Instruction::Call {
                    ident: ident as u32,
                    arg_count: u32::try_from(args.len()).expect("too many argument"),
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
                a.write_instructions(registry, out);
                b.write_instructions(registry, out);
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
                expr.write_instructions(registry, out);
                out.push(Instruction::Not);
            }
        }
    }
}

fn main() {
    let tokens = lex(b"1 + 10 / 5 * (3 - 4) + pow(2, 5)").unwrap();
    println!("{:?}", tokens.iter().map(|tk| tk.kind).collect::<Vec<_>>());

    let expr = parse(&tokens).unwrap();
    println!("{expr:#?}");

    let registry = Registry {
        vars: vec![(b"z".to_vec(), Value::Int(99))],
        fns: vec![(b"pow".to_vec(), builtin::pow)],
    };

    let mut ins_stream = Vec::new();
    expr.write_instructions(&registry, &mut ins_stream);

    println!("ins {ins_stream:#?}");

    let mut stack = Vec::new();
    for ins in ins_stream {
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

    println!("{stack:?}");
}

mod builtin {
    use crate::Value;

    pub fn pow(args: &[Value]) -> Value {
        match (args[0], args[1]) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a.pow(b as u32)),
            _ => todo!(),
        }
    }
}
