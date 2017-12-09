fn main() {
    let input = include_str!("input");
    println!("1 -> {}", score(input));
}

fn score(stream: &str) -> usize {
    let mut score = 0;
    let mut level = 0;

    let mut garbage = false;

    let mut chars = stream.chars();

    while let Some(ch) = chars.next() {
        match ch {
            '!' => { chars.next(); },

            '>' => garbage = false,
            _ if garbage => {},
            '<' => garbage = true,

            '{' => { level += 1 },
            '}' => { score += level; level -= 1; }

            _ => {},
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(score("{}"), 1);
        assert_eq!(score("{{{}}}"), 6);
        assert_eq!(score("{{},{}}"), 5);
        assert_eq!(score("{{{},{},{{}}}}"), 16);
        assert_eq!(score("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }
}
