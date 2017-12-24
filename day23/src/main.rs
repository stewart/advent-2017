use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("input").trim();

    let operations: Vec<Op> = input.
        lines().
        map(|line| line.parse().unwrap()).
        collect();

    let mut cpu = Cpu::new(operations.clone());

    cpu.run();

    println!("1 -> {}", cpu.muls());
}

struct Cpu {
    registers: HashMap<char, isize>,
    operations: Vec<Op>,
    idx: usize,
    muls: usize,
}

impl Cpu {
    fn new(operations: Vec<Op>) -> Self {
        let mut registers = HashMap::new();

        for ch in "abcdefgh".chars() {
            registers.insert(ch, 0);
        }

        Cpu {
            registers: registers,
            operations: operations,
            idx: 0,
            muls: 0,
        }
    }

    fn muls(&self) -> usize {
        self.muls
    }

    fn get(&self, value: Value) -> isize {
        use Value::*;

        match value {
            Reg(v) => {
                self.registers.get(&v).cloned().expect("Unexpected register")
            }
            Const(v) => v
        }
    }

    fn set(&mut self, register: char, value: isize) {
        self.registers.insert(register, value);
    }

    fn run(&mut self) {
        while self.idx < self.operations.len() {
            self.execute();
        }
    }

    fn execute(&mut self) {
        use Op::*;
        use Value::*;

        match self.operations[self.idx] {
            Set(Reg(a), b) => {
                let value = self.get(b);
                self.set(a, value);
                self.idx += 1;
            }

            Sub(Reg(a), b) => {
                let value = { self.get(Reg(a)) - self.get(b) };
                self.set(a, value);
                self.idx += 1;
            }

            Mul(Reg(a), b) => {
                let value = { self.get(Reg(a)) * self.get(b) };
                self.set(a, value);
                self.idx += 1;
                self.muls += 1;
            }

            Jnz(a, b) => {
                let a = self.get(a);
                let b = self.get(b);

                let offset: isize = if a == 0 { 1 } else { b };
                self.idx = (self.idx as isize + offset) as usize;
            }

            _ => panic!("Unrecognized instruction")
        }
    }
}

#[derive(Clone, Copy)]
pub enum Value {
    Reg(char),
    Const(isize)
}

impl FromStr for Value {
    type Err = String;

    fn from_str(input: &str) -> Result<Value, Self::Err> {
        match input.chars().next().unwrap() {
            ch @ 'a' ... 'z' => {
                Ok(Value::Reg(ch))
            }

            _ => {
                match input.parse() {
                    Ok(n) => Ok(Value::Const(n)),
                    _ => Err(format!("Unable to parse: {}", input))
                }
            }
        }
    }
}

#[derive(Clone)]
enum Op {
    Set(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Jnz(Value, Value),
}

impl FromStr for Op {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut tokens = input.split_whitespace();

        macro_rules! parse_two {
            ($op: expr) => {
                {
                    let a = tokens.next().expect("Arg A");
                    let b = tokens.next().expect("Arg B");

                    let a = a.parse().expect("Arg A");
                    let b = b.parse().expect("Arg B");

                    Ok($op(a, b))
                }
            }
        }

        match tokens.next().expect("Operation Name") {
            "set" => { parse_two!(Op::Set) }
            "sub" => { parse_two!(Op::Sub) }
            "mul" => { parse_two!(Op::Mul) }
            "jnz" => { parse_two!(Op::Jnz) }

            _ => Err(format!("Unable to parse: {}", input))
        }
    }
}
