const LEN: usize = 256;

fn main() {
    let input = include_str!("input").trim();

    let input = input.
        split(',').
        map(|i| i.parse::<usize>().unwrap()).
        collect::<Vec<usize>>();

    println!("1 -> {:?}", part1(&input));
}

fn part1(input: &[usize]) -> usize {
    let list = knots(input, 1);
    list[0] * list[1]
}

fn knots(input: &[usize], rounds: usize) -> Vec<usize> {
    let mut list = (0..LEN).collect::<Vec<_>>();
    let mut current = 0;
    let mut skip = 0;

    for _ in 0..rounds {
        for n in input {
            for i in 0..n / 2 {
                list.swap((current + i) % LEN, (current + n - 1 - i) % LEN);
            }

            current += n + skip;
            skip += 1;
        }
    }

    list
}
