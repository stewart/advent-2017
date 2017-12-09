fn main() {
    let input = include_str!("input");
    let (score, garbage) = score(input);

    println!("1 -> {}", score);
    println!("2 -> {}", garbage);
}

fn score(stream: &str) -> (usize, usize) {
    let mut score = 0;
    let mut level = 0;

    let mut garbage = false;
    let mut garbage_score = 0;

    let mut chars = stream.chars();

    while let Some(ch) = chars.next() {
        match ch {
            '!' => { chars.next(); },

            '>' => garbage = false,
            _ if garbage => { garbage_score += 1 },
            '<' => garbage = true,

            '{' => { level += 1 },
            '}' => { score += level; level -= 1; }

            _ => {},
        }
    }

    (score, garbage_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(score("{}"), (1, 0));
        assert_eq!(score("{{{}}}"), (6, 0));
        assert_eq!(score("{{},{}}"), (5, 0));
        assert_eq!(score("{{{},{},{{}}}}"), (16, 0));
        assert_eq!(score("{<a>,<a>,<a>,<a>}"), (1, 4));
        assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), (9, 8));
        assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), (9, 0));
        assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), (3, 17));
    }
}
