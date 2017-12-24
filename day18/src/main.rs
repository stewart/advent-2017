mod op;

use op::{Op, Value};

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Cpu {
    registers: HashMap<char, isize>,
    operations: Vec<Op>,
    queue: VecDeque<isize>,
    idx: usize,
    sent: usize,
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
            queue: VecDeque::new(),
            idx: 0,
            sent: 0,
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

    fn apply(&mut self, op: Op, other: &mut Self) {
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

            Snd(a) => {
                let value = self.get(a);
                other.queue.push_back(value);
                self.sent += 1;
                self.idx += 1;
            }

            Rcv(Reg(a)) => {
                if let Some(value) = self.queue.pop_front() {
                    self.set(a, value);
                    self.idx += 1;
                }
            }

            Set(Reg(a), b) => {
                let value = self.get(b);
                self.set(a, value);
                self.idx += 1;
            }

            _ => panic!("Unexpected op: {:?}", op)
        }
    }

    fn tick(&mut self, other: &mut Self) {
        let op = self.operations[self.idx].clone();
        self.apply(op, other);
    }
}

fn main() {
    let ops: Vec<Op> = include_str!("input").
        trim().
        lines().
        map(|line| line.parse().unwrap()).
        collect();

    let mut one = Cpu::new(0, ops.clone());
    let mut two = Cpu::new(1, ops.clone());

    loop {
        one.tick(&mut two);
        two.tick(&mut one);

        println!("Sent: {}", two.sent);
    }
}
