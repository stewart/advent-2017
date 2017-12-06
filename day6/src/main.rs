const INPUT: &str = "4  10  4   1   8   4   9   14  5   1   14  15  0   15  3   5";

fn main() {
    let input = INPUT.
        split_whitespace().
        map(|i| i.parse::<usize>().unwrap()).
        collect::<Vec<usize>>();

    println!("1 -> {:?}", one(input.clone()));
    println!("2 -> {:?}", two(input.clone()));
}

fn one(mut banks: Vec<usize>) -> usize {
    let mut seen_configurations: Vec<Vec<usize>> = vec![banks.clone()];

    'outer: loop {
        let (idx, mut blocks) = banks.iter().cloned().enumerate().rev().max_by_key(|&(_, v)| v).unwrap();
        banks[idx] = 0;

        for bank in banks.iter_mut().skip(idx + 1) {
            *bank += 1;
            blocks -= 1;

            if blocks == 0 {
                break;
            }
        }

        if blocks > 0 {
            'inner: loop {
                for bank in banks.iter_mut() {
                    *bank += 1;
                    blocks -= 1;

                    if blocks == 0 {
                        break 'inner;
                    }
                }
            }
        }

        seen_configurations.push(banks.clone());

        let mut a = seen_configurations.clone();
        a.sort();

        let mut b = a.clone();
        b.dedup();

        if a != b {
            break 'outer;
        }
    }

    seen_configurations.len() - 1
}

fn two(mut banks: Vec<usize>) -> usize {
    let mut seen_configurations: Vec<Vec<usize>> = vec![banks.clone()];

    fn iter(banks: &mut Vec<usize>, seen_configurations: &mut Vec<Vec<usize>>) {
        'outer: loop {
            let (idx, mut blocks) = banks.iter().cloned().enumerate().rev().max_by_key(|&(_, v)| v).unwrap();
            banks[idx] = 0;

            for bank in banks.iter_mut().skip(idx + 1) {
                *bank += 1;
                blocks -= 1;

                if blocks == 0 {
                    break;
                }
            }

            if blocks > 0 {
                'inner: loop {
                    for bank in banks.iter_mut() {
                        *bank += 1;
                        blocks -= 1;

                        if blocks == 0 {
                            break 'inner;
                        }
                    }
                }
            }

            seen_configurations.push(banks.clone());

            let mut a = seen_configurations.clone();
            a.sort();

            let mut b = a.clone();
            b.dedup();

            if a != b {
                break 'outer;
            }
        }
    }

    iter(&mut banks, &mut seen_configurations);
    seen_configurations = vec![banks.clone()];
    iter(&mut banks, &mut seen_configurations);
    seen_configurations.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(one(vec![0, 2, 7, 0]), 5);
    }

    #[test]
    fn test_two() {
        assert_eq!(two(vec![0, 2, 7, 0]), 4);
    }
}
