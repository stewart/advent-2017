extern crate day10;

use std::collections::{HashSet, VecDeque};

use day10::knots;

const INPUT: &str = "vbqugkhl";

type Grid = HashSet<(isize, isize)>;

fn main() {
    let grid = grid(INPUT);
    println!("1 -> {}", grid.len());
    println!("2 -> {}", regions(&grid));
}

fn grid(input: &str) -> Grid {
    (0..128).fold(HashSet::new(), |mut set, x| {
        let input = format!("{}-{}", input, x);

        knot(&input).iter().
            fold(String::new(), |row, n| format!("{}{:08b}", row, n)).
            chars().
            enumerate().
            filter(|&(_, ch)| ch == '1').
            map(|(y, _)| (x as isize, y as isize)).
            for_each(|coords| { set.insert(coords); });

        set
    })
}

fn regions(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let mut queue = VecDeque::new();
    let mut regions = 0;

    while let Some(&key) = grid.iter().next() {
        grid.remove(&key);
        queue.push_back(key);
        regions += 1;

        while let Some((x, y)) = queue.pop_front() {
            queue.extend(grid.take(&(x - 1, y)));
            queue.extend(grid.take(&(x, y - 1)));
            queue.extend(grid.take(&(x + 1, y)));
            queue.extend(grid.take(&(x, y + 1)));
        }
    }

    regions
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
