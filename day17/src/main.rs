const STEPS: usize = 369;

fn main() {
    println!("1 -> {}", part1());
    println!("2 -> {}", part2());
}

fn part1() -> usize {
    let mut ring = vec![0];
    let mut idx = 0;

    for value in 1..2018 {
        idx = 1 + (STEPS + idx) % ring.len();
        ring.insert(idx, value);
    }

    ring[(idx + 1) % ring.len()]
}

fn part2() -> usize {
    let mut ring = vec![0];
    let mut idx = 0;
    let mut len = 1;

    for value in 1..50_000_001 {
        idx = 1 + (STEPS + idx) % len;

        if idx == 1 {
            ring.insert(idx, value);
        }

        len += 1;

    }

    ring[1]
}
