use std::collections::HashMap;
use std::str::FromStr;

use grid::Grid;

#[derive(Debug, Clone)]
pub struct PatternSet {
    patterns: HashMap<Grid, Grid>,
}

impl PatternSet {
    pub fn new() -> Self {
        Self { patterns: HashMap::new() }
    }

    pub fn add_rule(&mut self, from: Grid, to: Grid) {
        for variant in from.permutations() {
            self.patterns.insert(variant, to.clone());
        }
    }

    pub fn apply_to(&self, from: &Grid) -> Grid {
        self.patterns.get(from).unwrap_or(&from).clone()
    }
}

impl FromStr for PatternSet {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut set = Self::new();

        for line in input.lines() {
            let mut grids = line.split(" => ").map(|grid| grid.parse().unwrap());

            let to = grids.next().unwrap();
            let from = grids.next().unwrap();

            set.add_rule(to, from);
        }

        Ok(set)
    }
}
