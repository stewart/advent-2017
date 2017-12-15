struct Generator {
    value: usize,
    factor: usize,
    multiplier: usize
}

impl Generator {
    fn new(value: usize, factor: usize, multiplier: usize) -> Generator {
        Generator { value: value, factor: factor, multiplier: multiplier }
    }

    fn next(&mut self) -> Option<u16> {
        self.value = (self.value * self.factor) % 2147483647;

        if self.value % self.multiplier == 0 {
            Some(self.value as u16)
        } else {
            None
        }
    }
}

fn main() {
    let mut a = Generator::new(116, 16807, 4);
    let mut b = Generator::new(299, 48271, 8);

    let mut matches = 0;

    for _ in 0..5_000_000 {
        let a_value: u16;
        let b_value: u16;

        loop {
            match a.next() {
                Some(a) => {
                    a_value = a;
                    break;
                },
                None => {}
            }
        }

        loop {
            match b.next() {
                Some(b) => {
                    b_value = b;
                    break;
                },
                None => {}
            }
        }

        if a_value == b_value {
            matches += 1;
        }
    }

    println!("2 -> {}", matches)
}
