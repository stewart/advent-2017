fn main() {
    let input = include_str!("input");
    println!("1 -> {}", bottom_program(input));
}

fn bottom_program(statements: &str) -> &str {
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_program() {
        assert_eq!(bottom_program(""), "");
    }
}
