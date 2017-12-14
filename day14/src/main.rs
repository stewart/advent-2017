extern crate day10;

use day10::knots;

const INPUT: &str = "vbqugkhl";

fn main() {
    println!("{:?}", knots(&[1, 2, 3, 4], 1));
    println!("{}", INPUT);
}
