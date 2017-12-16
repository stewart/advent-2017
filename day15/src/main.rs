fn main() {
    println!("1 -> {}", part1(65, 48271));
    println!("2 -> {}", part2(65, 48271));
}

fn part1(mut a: usize, mut b: usize) -> usize {
    (0..40_000_000).filter(|_| {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;
        a as u16 == b as u16
    }).count()
}

fn part2(mut a: usize, mut b: usize) -> usize {
    (0..5_000_000).filter(|_| {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;

        while a % 4 != 0 {
            a = (a * 16807) % 2147483647;
        }

        while b % 8 != 0 {
            b = (b * 48271) % 2147483647;
        }

        a as u16 == b as u16
    }).count()
}
