use std::collections::HashMap;

type Registers = HashMap<String, isize>;

use Op::*;
use Cond::*;

enum Op {
    Inc(String, isize),
    Dec(String, isize)
}

impl Op {
    fn parse(symbol: &str, register: String, value: isize) -> Op {
        match symbol {
            "inc" => Inc(register, value),
            "dec" =>  Dec(register, value),
            _ => panic!("unexpected op: {}", symbol)
        }
    }

    fn apply(&self, registers: &mut Registers) {
        let register = match *self {
            Inc(ref r, _) | Dec(ref r, _) => r
        };

        let register = registers.
            entry(register.to_string()).
            or_insert(0);

        match *self {
            Inc(_, value) => *register += value,
            Dec(_, value) => *register -= value
        };
    }
}

enum Cond {
    Eq(String, isize),
    NotEq(String, isize),
    GreaterThan(String, isize),
    LessThan(String, isize),
    GreaterThanOrEqualTo(String, isize),
    LessThanOrEqualTo(String, isize),
}

impl Cond {
    fn parse(symbol: &str, register: String, value: isize) -> Cond {
        match symbol {
            "==" => Eq(register, value),
            "!=" => NotEq(register, value),
            ">" => GreaterThan(register, value),
            "<" => LessThan(register, value),
            ">=" => GreaterThanOrEqualTo(register, value),
            "<=" => LessThanOrEqualTo(register, value),
            _ => panic!("unexpected condition: {}", symbol)
        }
    }

    fn test(&self, registers: &mut Registers) -> bool {
        let register = match *self {
            Eq(ref r, _) |
            NotEq(ref r, _) |
            GreaterThan(ref r, _) |
            LessThan(ref r, _) |
            GreaterThanOrEqualTo(ref r, _) |
            LessThanOrEqualTo(ref r, _) => r
        };

        let register = registers.
            entry(register.to_string()).
            or_insert(0);

        match *self {
            Eq(_, value)                   => *register == value,
            NotEq(_, value)                => *register != value,
            GreaterThan(_, value)          => *register > value,
            LessThan(_, value)             => *register < value,
            GreaterThanOrEqualTo(_, value) => *register >= value,
            LessThanOrEqualTo(_, value)    => *register <= value
        }
    }
}

struct Instruction(Op, Cond);

fn main() {
    let input = include_str!("input");
    let instructions = parse(input);

    let mut registers = HashMap::new();

    let max = run(&mut registers, instructions);

    println!("1 -> {:?}", registers.values().max().unwrap());
    println!("2 -> {:?}", max);
}

fn parse(instructions: &str) -> Vec<Instruction> {
    instructions.
        lines().
        map(str::split_whitespace).
        map(|mut line| {
            let register = line.next().expect("Initial register");
            let register = String::from(register);
            let op = line.next().expect("Operation");

            let value = line.next().expect("Operation Value");
            let value = value.parse::<isize>().expect("Parsing value");

            let op = Op::parse(op, register, value);

            line.next().expect("if");

            let register = line.next().expect("Conditional register");
            let register = String::from(register);

            let cond = line.next().expect("Conditional operation");

            let value = line.next().expect("Conditional value");
            let value = value.parse::<isize>().expect("Parsing conditional");

            let cond = Cond::parse(cond, register, value);

            Instruction(op, cond)
        }).collect()
}

fn run(registers: &mut Registers, instructions: Vec<Instruction>) -> isize {
    let mut max: isize = 0;

    for instruction in instructions {
        let ref operation = instruction.0;
        let ref condition = instruction.1;

        if condition.test(registers) {
            operation.apply(registers);
        }

        max = max.max(*registers.values().max().expect("maximum value"));
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(1, 1);
    }
}
