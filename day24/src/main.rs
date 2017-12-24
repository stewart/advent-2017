use std::collections::VecDeque;
use std::str::FromStr;

// path, next, parts
type Queue = VecDeque<(Vec<Part>, usize, Vec<Part>)>;

fn main() {
    let parts = load_parts();
    let mut queue: Queue = VecDeque::new();

    // initial entries
    for (index, _) in parts.iter().enumerate().filter(|&(_, ref p)| p.can_connect(0)) {
        let mut parts = parts.clone();
        let part = parts.remove(index);
        queue.push_back((vec![part], part.other(0), parts));
    }

    let mut best = 0;
    let mut longest = (0, 0);

    while let Some((path, slot, parts)) = queue.pop_front() {
        if parts.len() == 0 {
            continue;
        }

        let mut any = false;

        for (index, _) in parts.iter().enumerate().filter(|&(_, ref p)| p.can_connect(slot)) {
            any = true;

            let mut parts = parts.clone();
            let part = parts.remove(index);

            let mut path = path.clone();
            path.push(part);

            queue.push_back((path, part.other(slot), parts));
        }

        // no possibilities found, calculate strength
        if !any {
            let strength = path.iter().map(Part::value).sum();
            best = best.max(strength);

            let length = path.len();

            if length >= longest.0 {
                longest = if length > longest.0 {
                    (length, strength)
                } else {
                    let strength = longest.1.max(strength);
                    (length, strength)
                }
            }
        }
    }

    println!("1 -> {}", best);
    println!("2 -> {}", longest.1);
}

fn load_parts() -> Vec<Part> {
    include_str!("input").
        trim().
        lines().
        map(|line| line.parse().unwrap()).
        collect()
}

#[derive(Debug, Clone, Copy)]
struct Part {
    left: usize,
    right: usize,
}

impl Part {
    fn value(&self) -> usize {
        self.left + self.right
    }

    fn can_connect(&self, prev: usize) -> bool {
        prev == self.left || prev == self.right
    }

    fn other(&self, value: usize) -> usize {
        if self.left == value { self.right } else { self.left }
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input = input.split("/");

        let left = input.next().expect("Left");
        let left = left.parse().expect("Left");

        let right = input.next().expect("Right");
        let right = right.parse().expect("Right");

        Ok(Part { left: left, right: right })
    }
}

