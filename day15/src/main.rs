struct Generator {
    value: usize,
    factor: usize
}

impl Generator {
    fn new(value: usize, factor: usize) -> Generator {
        Generator { value: value, factor: factor }
    }

    fn next(&mut self) -> u16 {
        self.value = (self.value * self.factor) % 2147483647;
        self.value as u16
    }
}

struct PickyGenerator {
    value: usize,
    factor: usize,
    multiplier: usize
}

impl PickyGenerator {
    fn new(value: usize, factor: usize, multiplier: usize) -> PickyGenerator {
        PickyGenerator { value: value, factor: factor, multiplier: multiplier }
    }

    fn next(&mut self) -> u16 {
        loop {
            self.value = (self.value * self.factor) % 2147483647;
            if self.value % self.multiplier == 0 {
                return self.value as u16;
            }
        }
    }
}

fn main() {
    println!("1 -> {}", part1());
    println!("2 -> {}", part2());
}

fn part1() -> usize {
    let mut a = Generator::new(116, 16807);
    let mut b = Generator::new(299, 48271);

    let mut matches = 0;

    for _ in 0..40_000_000 {
        if a.next() == b.next() {
            matches += 1;
        }
    }

    matches
}

fn part2() -> usize {
    let mut a = PickyGenerator::new(116, 16807, 4);
    let mut b = PickyGenerator::new(299, 48271, 8);

    let mut matches = 0;

    for _ in 0..5_000_000 {
        if a.next() == b.next() {
            matches += 1;
        }
    }

    matches
}
