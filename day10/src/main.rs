extern crate day10;

use day10::knots;

fn main() {
    let input = include_str!("input").trim();
    println!("1 -> {:?}", part1(&input));
    println!("2 -> {:?}", part2(&input));
}

fn part1(input: &str) -> usize {
    let input = input.
        split(',').
        map(|i| i.parse::<usize>().unwrap()).
        collect::<Vec<usize>>();

    let list = knots(&input, 1);
    list[0] * list[1]
}

fn part2(input: &str) -> String {
    let mut input = input.
        bytes().
        map(|b| b as usize).
        collect::<Vec<_>>();

    input.extend(vec![17, 31, 73, 47, 23]);

    let list = knots(&input, 64).
        chunks(16).
        map(|chunk| format!("{:02x}", chunk.iter().fold(0, |a, b| a ^ b))).
        collect::<Vec<_>>();

    list.join("")
}
