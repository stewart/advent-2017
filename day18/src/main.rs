mod op;

use op::{Op, Value};

use std::collections::{HashMap};

#[derive(Debug)]
struct Cpu {
    registers: HashMap<char, isize>,
    operations: Vec<Op>,
    idx: usize,
    played: isize,
}

impl Cpu {
    fn new(initial_value: isize, operations: Vec<Op>) -> Cpu {
        let mut registers = HashMap::new();

        for ch in "abcdefghijklmnop".chars() {
            registers.insert(ch, 0);
        }

        registers.insert('p', initial_value);

        Cpu {
            registers: registers,
            operations: operations,
            idx: 0,
            played: 0,
        }
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

    fn apply(&mut self, op: Op) {
        use Op::*;
        use Value::*;

        match op {
            Add(Reg(a), b) => {
                let value = self.get(Reg(a)) + self.get(b);
                self.set(a, value);
                self.idx += 1;
            }

            Jgz(a, b) => {
                let a = self.get(a);
                let b = self.get(b);

                let jump: isize = if a > 0 { b } else { 1 };
                self.idx = (self.idx as isize + jump) as usize;
            }

            Mod(Reg(a), b) => {
                let value = self.get(Reg(a)) % self.get(b);
                self.set(a, value);
                self.idx += 1;
            }

            Mul(Reg(a), b) => {
                let value = self.get(Reg(a)) * b;
                self.set(a, value);
                self.idx += 1;
            }

            Rcv(a) => {
                if self.get(a) != 0 {
                    println!("Recovered: {}", self.played);
                }
                self.idx += 1;
            }

            Set(Reg(a), b) => {
                let value = self.get(b);
                self.set(a, value);
                self.idx += 1;
            }

            Snd(a) => {
                let value = self.get(a);
                self.played = value;
                self.idx += 1;
            }

            _ => panic!("Unexpected op: {:?}", op)
        }
    }

    fn run(&mut self) {
        while self.idx >= 0 && self.idx < self.operations.len() {
            let op = self.operations[self.idx].clone();
            self.apply(op);
        }
    }
}

fn main() {
    let ops: Vec<Op> = include_str!("input").
        trim().
        lines().
        map(|line| line.parse().unwrap()).
        collect();

    let mut cpu = Cpu::new(0, ops.clone());

    cpu.run();
}
