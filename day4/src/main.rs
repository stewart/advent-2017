fn main() {
    let input = include_str!("input");
    let lines = input.lines();

    let one = input.lines().filter(|line| is_valid(line)).count();
    let two = input.lines().filter(|line| valid_with_anagrams(line)).count();

    println!("1 -> {}", one);
    println!("2 -> {}", two);
}

fn is_valid(s: &str) -> bool {
    let mut x: Vec<&str> = s.split_whitespace().collect();
    x.sort();

    let mut y = x.clone();
    y.dedup();

    x == y
}

fn valid_with_anagrams(s: &str) -> bool {
    let mut x = s.split_whitespace().
        map(|word| {
            let mut w = word.chars().collect::<Vec<_>>(); w.sort(); w
        }).
        collect::<Vec<Vec<_>>>();

    x.sort();

    let mut y = x.clone();
    y.dedup();

    x == y
}

fn valid_unique(k)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid("aa bb cc dd ee"));
        assert!(!is_valid("aa bb cc dd aa"));
        assert!(is_valid("aa bb cc dd aaa"));
    }

    fn test_is_valid_with_anagrams() {
        assert!(valid_with_anagrams("abcde fghij"));
        assert!(!valid_with_anagrams("abcde xyz ecdab"));
        assert!(valid_with_anagrams("a ab abc abd abf abj"));
        assert!(valid_with_anagrams("iiii oiii ooii oooi oooo"));
        assert!(!valid_with_anagrams("oiii ioii iioi iiio"));
    }
}
