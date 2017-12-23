use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

fn main() {
    let input = include_str!("input").trim();
    let mut cluster: Cluster = input.parse().unwrap();

    for _ in 0..10_000_000 {
        cluster.tick();
    }

    println!("Infections: {}", cluster.infections);
}

#[derive(Clone, PartialEq)]
enum State { Clean, Infected, Weakened, Flagged }

#[derive(Clone)]
enum Heading { Up, Down, Left, Right }

type Coordinates = (isize, isize);

impl std::ops::Add<Heading> for Coordinates {
    type Output = Self;

    fn add(self, rhs: Heading) -> Self {
        match rhs {
            Heading::Up    => (self.0, self.1 - 1),
            Heading::Down  => (self.0, self.1 + 1),
            Heading::Left  => (self.0 - 1, self.1),
            Heading::Right => (self.0 + 1, self.1),
        }
    }
}

#[test]
fn test_coordinates_add() {
    use Heading::*;
    assert_eq!((0, 0) + Up, (0, -1));
    assert_eq!((0, 0) + Down, (0, 1));
    assert_eq!((0, 0) + Left, (-1, 0));
    assert_eq!((0, 0) + Right, (1, 0));
}

struct Virus {
    pub coordinates: Coordinates,
    heading: Heading,
}

impl Virus {
    fn new(x: isize, y: isize) -> Self {
        Virus { coordinates: (x, y), heading: Heading::Up }
    }

    fn move_forward(&mut self) {
        self.coordinates = self.coordinates + self.heading.clone();
    }

    fn turn_right(&mut self) {
        use Heading::*;

        self.heading = match self.heading {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn turn_left(&mut self) {
        use Heading::*;

        self.heading = match self.heading {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

struct Cluster {
    pub infections: usize,
    data: HashMap<Coordinates, State>,
    virus: Virus,
}

impl Cluster {
    fn tick(&mut self) {
        use State::*;

        let status = {
            self.data.get(&self.virus.coordinates).cloned().unwrap_or(Clean)
        };

        match status {
            Clean => {
                self.virus.turn_left()
            }

            Weakened => {
            }

            Infected => {
                self.virus.turn_right()
            }

            Flagged => {
                self.virus.turn_right();
                self.virus.turn_right();
            }
        }

        let status = match status {
            Clean => Weakened,
            Weakened => Infected,
            Infected => Flagged,
            Flagged => Clean
        };

        if status == Infected {
            self.infections += 1;
        }

        self.data.insert(self.virus.coordinates, status);
        self.virus.move_forward();
    }
}

impl FromStr for Cluster {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let width = input.chars().position(|c| c == '\n').unwrap();
        let height = input.lines().count();

        let mut data = HashMap::new();
        let virus = Virus::new((width / 2) as isize, (height / 2) as isize);

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                data.insert((x as isize, y as isize), match ch {
                    '#' => State::Infected,
                    _ => State::Clean,
                });
            }
        }

        Ok(Cluster { data: data, virus: virus, infections: 0 })
    }
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let xs: Vec<isize> = self.data.keys().map(|&(x, _)| x).collect();
        let ys: Vec<isize> = self.data.keys().map(|&(_, y)| y).collect();

        let xmax = xs.iter().max().cloned().unwrap_or(0);
        let xmin = xs.iter().min().cloned().unwrap_or(0);

        let ymax = ys.iter().max().cloned().unwrap_or(0);
        let ymin = ys.iter().min().cloned().unwrap_or(0);

        for y in ymin..(ymax + 1) {
            for x in xmin..(xmax + 1) {
                let x = x as isize;
                let y = y as isize;

                let value = match self.data.get(&(x, y)) {
                    Some(&State::Infected) => '#',
                    Some(&State::Weakened) => 'W',
                    Some(&State::Flagged) => 'F',
                    _ => '.'
                };

                if self.virus.coordinates == (x, y) {
                    write!(f, "[{}]", value)?;
                } else {
                    write!(f, " {} ", value)?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}
