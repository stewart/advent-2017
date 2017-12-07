#![feature(string_retain)]

use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
struct Program {
    name: String,
    weight: usize,
    children: Vec<String>
}

impl Program {
    fn new(mut program: String) -> Program {
        program.retain(|ch| {
            match ch {
                '(' | ')' | '-' | '>' | ',' => false,
                _ => true
            }
        });

        let program = program.split_whitespace().collect::<Vec<_>>();

        let program = Program {
            name: String::from(program[0]),
            weight: program[1].parse().unwrap(),
            children: program.into_iter().skip(2).map(String::from).collect()
        };

        program
    }
}

impl PartialOrd for Program {
    fn partial_cmp(&self, other: &Program) -> Option<Ordering> {
        if self.children.contains(&other.name) {
            println!("{} supports {}", self.name, other.name);
            return Some(Ordering::Less);
        }

        if other.children.contains(&self.name) {
            println!("{} supports {}", other.name, self.name);
            return Some(Ordering::Greater);
        }

        Some(Ordering::Equal)
    }
}

fn main() {
    let input = include_str!("input");
    println!("1 -> {}", bottom_program(input));
}

fn bottom_program(statements: &str) -> String {
    let programs = statements.
        lines().
        map(String::from).
        map(Program::new).
        collect::<Vec<_>>();

    let bottom = programs.iter().find(|program| {
        !programs.iter().any(|other| other.children.contains(&program.name))
    }).unwrap();

    bottom.name.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_program() {
        assert_eq!(bottom_program("pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"), "tknk");
    }
}
