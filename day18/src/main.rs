mod op;

use op::{Op, Value};

fn main() {
    let ops: Vec<Op> = include_str!("input").
        trim().
        lines().
        map(|line| line.parse().unwrap()).
        collect();

    println!("1 -> {:?}", ops);
}
