use std::collections::HashMap;

const INPUT: &str = "4  10  4   1   8   4   9   14  5   1   14  15  0   15  3   5";

fn main() {
    let input = INPUT.
        split_whitespace().
        map(|i| i.parse::<usize>().unwrap()).
        collect::<Vec<usize>>();

    let (cycles, length) = solve(input);

    println!("1 -> {:?}", cycles);
    println!("2 -> {:?}", length);
}

fn solve(mut banks: Vec<usize>) -> (usize, usize) {
    let len = banks.len();
    let mut seen = HashMap::new();
    let mut count = 0;

    while !seen.contains_key(&banks) {
        seen.insert(banks.clone(), count);

        let (i, &val) = banks.iter().
            enumerate().
            max_by_key(|&(i, val)| (val, -(i as isize))).
            unwrap();

        banks[i] = 0;

        for val in 0..(val as usize) {
            let idx = (i + 1 + val) % len;
            banks[idx] += 1;
        }

        count += 1;
    }

    (count, count - seen.get(&banks).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(vec![0, 2, 7, 0]), (5, 4));
    }
}
