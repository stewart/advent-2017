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
            registers.insert(ch, initial_value);
        }

        Cpu {
            registers: registers,
            operations: operations,
            idx: 0,
            played: 0,
        }
    }
}

fn main() {
    let ops: Vec<Op> = include_str!("input").
        trim().
        lines().
        map(|line| line.parse().unwrap()).
        collect();

    let cpu = Cpu::new(0, ops.clone());

    println!("1 -> {:?}", cpu);
}
