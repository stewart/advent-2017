fn main() {
    let input = include_str!("input").trim();

    let layers: Vec<(usize, usize)> = input.lines().map(parse).collect();

    let severity: usize = layers.iter().
        filter_map(|&(depth, range)| {
            if caught(depth, range) {
                Some(depth * range)
            } else {
                None
            }
        }).sum();

    println!("1 -> {}", severity);

    let wait: usize = (0..).find(|delay| {
        !layers.iter().any(|&(depth, range)| caught(delay + depth, range))
    }).expect("Wait");

    println!("2 -> {}", wait);
}

fn parse(line: &str) -> (usize, usize) {
    let mut line = line.
        split(": ").
        map(|s| s.parse().unwrap());

    let depth = line.next().expect("Depth");
    let range = line.next().expect("Range");

    (depth, range)
}

fn caught(depth: usize, range: usize) -> bool {
    depth % (2 * (range - 1)) == 0
}
