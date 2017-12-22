mod grid;
mod pattern_set;

use grid::Grid;
use pattern_set::PatternSet;

fn main() {
    let rules: PatternSet = include_str!("input").trim().parse().unwrap();
    println!("1 -> {}", solve(rules.clone(), 5));
    println!("2 -> {}", solve(rules.clone(), 18));
}

fn solve(patterns: PatternSet, iterations: usize) -> usize {
    let mut art = Grid::default();

    for _ in 0..iterations {
        let grids: Vec<Grid> = art.
            split().
            iter().
            map(|grid| patterns.apply_to(grid)).
            collect();

        art = Grid::stitch(grids);
    }

    art.count()
}
