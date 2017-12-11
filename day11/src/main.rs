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
    println!("1 -> {}", part1(input));
}

fn part1(input: &str) -> isize {
    let end = input.
        split(",").
        fold(Point(0, 0), |position, direction| {
            position + match direction {
                "nw" => Point(-1, 1),
                "n" => Point(0, 1),
                "ne" => Point(1, 0),

                "sw" => Point(-1, 0),
                "s" => Point(0, -1),
                "se" => Point(1, -1),

                _ => panic!("Unexpected direction - {}", direction)
            }
        });

    end.distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("ne,ne,ne"), 3);
        assert_eq!(part1("ne,ne,sw,sw"), 0);
        assert_eq!(part1("ne,ne,s,s"), 2);
        assert_eq!(part1("se,sw,se,sw,sw"), 3);
    }
}
