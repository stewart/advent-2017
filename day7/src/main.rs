fn main() {
    println!("Hello, world!");
}

fn something() -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(something(), 1);
    }
}
