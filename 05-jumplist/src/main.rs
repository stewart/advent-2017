fn main() {
    let input = include_str!("input");

    let input = input.
        lines().
        map(|i| i.parse().unwrap()).
        collect::<Vec<i32>>();

    println!("1 -> {}", escape(input.clone(), false));
    println!("2 -> {}", escape(input.clone(), true));
}

fn escape(mut jmplist: Vec<i32>, weird_offsets: bool) -> usize {
    let mut idx: i32 = 0;
    let mut steps = 0;

    while let Some(value) = jmplist.get_mut(idx as usize) {
        idx += *value;
        *value += if weird_offsets && *value >= 3 { -1 } else { 1 };
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape() {
        assert_eq!(escape(vec![0, 3, 0, 1, -3], false), 5);
        assert_eq!(escape(vec![0, 3, 0, 1, -3], true), 10);
    }
}
