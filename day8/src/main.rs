use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    Inc(String, isize),
    Dec(String, isize)
}

#[derive(Debug)]
enum Cond {
    Eq(String, isize),
    NotEq(String, isize),
    GreaterThan(String, isize),
    LessThan(String, isize),
    GreaterThanOrEqualTo(String, isize),
    LessThanOrEqualTo(String, isize),
}

#[derive(Debug)]
struct Instruction(Op, Cond);

fn main() {
    let input = include_str!("input");
    let instructions = parse(input);

    let mut registers = HashMap::new();

    run(&mut registers, instructions);

    println!("1 -> {:?}", registers.values().max());
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

            let op = match op {
                "inc" => Op::Inc(register, value),
                "dec" =>  Op::Dec(register, value),
                _ => panic!("Unexpected operation received")
            };

            line.next().expect("if");

            let register = line.next().expect("Conditional register");
            let register = String::from(register);

            let cond = line.next().expect("Conditional operation");

            let value = line.next().expect("Conditional value");
            let value = value.parse::<isize>().expect("Parsing conditional");

            let cond = match cond {
                "==" => Cond::Eq(register, value),
                "!=" => Cond::NotEq(register, value),
                ">" => Cond::GreaterThan(register, value),
                "<" => Cond::LessThan(register, value),
                ">=" => Cond::GreaterThanOrEqualTo(register, value),
                "<=" => Cond::LessThanOrEqualTo(register, value),
                _ => panic!("Unexpected condition received")
            };

            Instruction(op, cond)
        }).collect()
}

fn run(registers: &mut HashMap<String, isize>, instructions: Vec<Instruction>) {
    for instruction in instructions {
        let ref operation = instruction.0;
        let ref condition = instruction.1;

        let (register, comparison) = match condition {
            &Cond::Eq(ref register, value) => (register, value),
            &Cond::NotEq(ref register, value) => (register, value),
            &Cond::GreaterThan(ref register, value) => (register, value),
            &Cond::LessThan(ref register, value) => (register, value),
            &Cond::GreaterThanOrEqualTo(ref register, value) => (register, value),
            &Cond::LessThanOrEqualTo(ref register, value) => (register, value)
        };


        let conditional_passes = {
            let value = registers.entry(register.clone()).or_insert(0);

            match condition {
                &Cond::Eq(_, _) => { *value == comparison },
                &Cond::NotEq(_, _) => { *value != comparison }
                &Cond::GreaterThan(_, _) => { *value > comparison },
                &Cond::LessThan(_, _) =>  { *value < comparison },
                &Cond::GreaterThanOrEqualTo(_, _) => { *value >= comparison },
                &Cond::LessThanOrEqualTo(_, _) => { *value <= comparison }
            }
        };

        if conditional_passes {
            match operation {
                &Op::Inc(ref register, value) => {
                    let register = registers.entry(register.clone()).or_insert(0);
                    *register += value
                },

                &Op::Dec(ref register, value) => {
                    let register = registers.entry(register.clone()).or_insert(0);
                    *register -= value
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(1, 1);
    }
}
