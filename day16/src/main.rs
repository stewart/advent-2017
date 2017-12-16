#![feature(slice_rotate)]

const DANCERS: [char; 16] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
];

use Op::*;

#[derive(Debug)]
enum Op {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

impl Op {
    fn parse(input: &str) -> Op {
        let mut input = input.chars();
        let op = input.next().unwrap();

        let input: String = input.collect();
        let mut input = input.split("/");

        let parse = |n: &str| -> usize { n.parse().unwrap() };
        let get_char = |s: &str| -> Option<char> { s.chars().next() };

        match op {
            'x' => {
                let a: usize = input.next().map(&parse).expect("Partner A");
                let b: usize = input.next().map(&parse).expect("Partner B");
                Exchange(a, b)
            },

            'p' => {
                let a = input.next().and_then(&get_char).expect("Partner A");
                let b = input.next().and_then(&get_char).expect("Partner B");
                Partner(a, b)
            },

            's' => {
                let n: usize = input.next().map(&parse).expect("Spin Count");
                Spin(n)
            }

            op => { panic!("Unknown op found: {}", op); }
        }
    }

    fn apply(&self, dancers: &mut Vec<char>) {
        match *self {
            Exchange(a, b) => {
                let mut dancers = dancers.as_mut_slice();
                dancers.swap(a, b);
            },

            Partner(a, b) => {
                let a = dancers.iter().position(|&c| c == a).unwrap();
                let b = dancers.iter().position(|&c| c == b).unwrap();
                dancers.swap(a, b)
            },

            Spin(n) => {
                let idx = dancers.len() - n;
                let dancers = dancers.as_mut_slice();
                dancers.rotate(idx);
            }
        }
    }
}

type Operations = Vec<Op>;

fn main() {
    let operations: Operations = load_operations();

    println!("1 -> {}", part1(&operations));
    println!("2 -> {}", part2(&operations));
}

fn load_operations() -> Operations {
    include_str!("input").trim().split(",").map(Op::parse).collect()
}

fn cycle_count(ops: &Operations) -> usize {
    let initial = DANCERS.to_vec();
    let mut dancers = initial.clone();
    let mut cycle = 1;

    loop {
        for op in ops {
            op.apply(&mut dancers);
        }

        if dancers == initial {
            break;
        } else {
            cycle += 1;
        }
    }

    cycle
}

fn part1(operations: &Operations) -> String {
    let mut dancers = DANCERS.to_vec();

    for op in operations {
        op.apply(&mut dancers);
    }

    dancers.into_iter().collect()
}

fn part2(operations: &Operations) -> String {
    let mut dancers = DANCERS.to_vec();

    for _ in 0..(1_000_000_000 % cycle_count(operations)) {
        for op in operations {
            op.apply(&mut dancers);
        }
    }

    dancers.into_iter().collect()
}
