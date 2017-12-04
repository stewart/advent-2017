macro_rules! check_validity {
    ($input: ident, $fn: ident) => {
        $input.lines().filter(|l| $fn(l)).count()
    }
}

fn main() {
    let input = include_str!("input");

    println!("1 -> {}", check_validity!(input, uniqueness));
    println!("2 -> {}", check_validity!(input, anagrams));
}

fn uniqueness(s: &str) -> bool {
    let mut x = s.split_whitespace().collect::<Vec<_>>();
    x.sort();

    let mut y = x.clone();
    y.dedup();

    x == y
}

fn anagrams(s: &str) -> bool {
    let mut x = s.
        split_whitespace().
        map(|word| {
            let mut word = word.chars().collect::<Vec<_>>();
            word.sort();
            word
        }).collect::<Vec<_>>();
    x.sort();

    let mut y = x.clone();
    y.dedup();

    x == y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniqueness() {
        assert!(uniqueness("aa bb cc dd ee"));
        assert!(!uniqueness("aa bb cc dd aa"));
        assert!(uniqueness("aa bb cc dd aaa"));
    }

    #[test]
    fn test_anagrams() {
        assert!(anagrams("abcde fghij"));
        assert!(!anagrams("abcde xyz ecdab"));
        assert!(anagrams("a ab abc abd abf abj"));
        assert!(anagrams("iiii oiii ooii oooi oooo"));
        assert!(!anagrams("oiii ioii iioi iiio"));
    }
}
