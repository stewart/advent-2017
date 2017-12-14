const LEN: usize = 256;

pub fn knots(input: &[usize], rounds: usize) -> Vec<usize> {
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
