use Op::*;

#[derive(Debug)]
enum Op {
    // Move programs from back to front
    Spin(usize),

    // Programs at A/B swap places
    Exchange(usize, usize),

    // Programs at A/B swap places
    Partner(char, char)
}

impl Op {
    fn parse(input: &str) -> Op {
        let mut input = input.chars();
        let op = input.next().unwrap();

        let input: String = input.collect();
        let mut input = input.split("/");

        match op {
            'x' => {
                let a: usize = input.next().map(|n| n.parse().unwrap()).expect("Partner A");
                let b: usize = input.next().map(|n| n.parse().unwrap()).expect("Partner B");
                Exchange(a, b)
            },

            'p' => {
                let a = input.next().and_then(|ch| ch.chars().next()).expect("Partner A");
                let b = input.next().and_then(|ch| ch.chars().next()).expect("Partner B");
                Partner(a, b)
            },

            's' => {
                let n: usize = input.next().map(|n| n.parse().unwrap()).expect("Spin Count");
                Spin(n)
            }

            op => { panic!("Unknown op found: {}", op); }
        }
    }

    fn apply(&self, mut dancers: Vec<char>) -> Vec<char> {
        match *self {
            Exchange(a, b) => {
                let mut dancers = dancers.as_mut_slice();
                dancers.swap(a, b);
            },
            Partner(a, b) => {
                let (a, _) = dancers.iter().enumerate().find(|&(_, c)| *c == a).unwrap();
                let (b, _) = dancers.iter().enumerate().find(|&(_, c)| *c == b).unwrap();
                dancers.swap(a, b)
            },
            Spin(n) => {
                let d = dancers.clone();
                let (a, b) = d.split_at(d.len() - n);
                let mut b = b.to_vec();
                b.extend_from_slice(a);
                dancers = b;
            }
        }

        dancers
    }
}

fn main() {
    let dancers = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];

    let operations: Vec<Op> = include_str!("input").
        trim().
        split(",").
        map(Op::parse).
        collect();

    part1(&operations, &dancers);
}

fn part1(operations: &Vec<Op>, dancers: &Vec<char>) {
    let mut dancers = dancers.clone();

    for op in operations {
        dancers = op.apply(dancers);
    }

    println!("Dancers: {:?}", dancers.iter().collect::<String>());
}
