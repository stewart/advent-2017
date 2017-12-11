use std::ops::Add;

#[derive(Debug)]
struct Point(isize, isize);

impl Point {
    fn distance(&self) -> isize {
        (self.0.abs() + self.1.abs() + (self.0 + self.1).abs()) / 2
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

fn main() {
    let input = include_str!("input").trim();
    let (part1, part2) = solve(input);
    println!("1 -> {}", part1);
    println!("2 -> {}", part2);
}

fn solve(input: &str) -> (isize, isize) {
    let (position, max) = input.
        split(",").
        fold((Point(0, 0), 0), |(position, max), direction| {
            let new = position + match direction {
                "nw" => Point(-1, 1),
                "n" => Point(0, 1),
                "ne" => Point(1, 0),

                "sw" => Point(-1, 0),
                "s" => Point(0, -1),
                "se" => Point(1, -1),

                _ => panic!("Unexpected direction - {}", direction)
            };

            let max = max.max(new.distance());

            (new, max)
        });

    (position.distance(), max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve("ne,ne,ne"), (3, 3));
        assert_eq!(solve("ne,ne,sw,sw"), (0, 2));
        assert_eq!(solve("ne,ne,s,s"), (2, 2));
        assert_eq!(solve("se,sw,se,sw,sw"), (3, 3));
    }
}
