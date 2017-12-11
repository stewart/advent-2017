const LEN: usize = 256;

fn main() {
    let input = include_str!("input").trim();
    println!("1 -> {:?}", part1(&input));
    println!("2 -> {:?}", part2(&input));
}

fn part1(input: &str) -> usize {
    let input = input.
        split(',').
        map(|i| i.parse::<usize>().unwrap()).
        collect::<Vec<usize>>();

    let list = knots(&input, 1);
    list[0] * list[1]
}

fn part2(input: &str) -> String {
    let mut input = input.
        bytes().
        map(|b| b as usize).
        collect::<Vec<_>>();

    input.extend(vec![17, 31, 73, 47, 23]);

    let list = knots(&input, 64).
        chunks(16).
        map(|chunk| format!("{:02x}", chunk.iter().fold(0, |a, b| a ^ b))).
        collect::<Vec<_>>();

    list.join("")
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
