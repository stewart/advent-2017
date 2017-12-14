extern crate day10;

use day10::knots;

const INPUT: &str = "vbqugkhl";

fn main() {
    let grid = grid(INPUT);

    let squares_used: usize = grid.iter().
        map(|row| row.chars().filter(|&c| c == '1').count()).
        sum();

    println!("{:?}", squares_used);
}

fn grid(input: &str) -> Vec<String> {
    (0..128).map(|n| {
        let input = format!("{}-{}", input, n);
        let row = String::new();
        knot(&input).iter().fold(row, |row, n| format!("{}{:08b}", row, n))
    }).collect()
}

fn knot(input: &str) -> Vec<usize> {
    let mut input = input.
        bytes().
        map(|b| b as usize).
        collect::<Vec<_>>();

    input.extend(vec![17, 31, 73, 47, 23]);

    knots(&input, 64).
        chunks(16).
        map(|chunk| chunk.into_iter().fold(0, |a, b| a ^ b)).
        collect()
}
