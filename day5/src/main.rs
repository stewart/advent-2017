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
    let len = jmplist.len() as i32;
    let mut idx: i32 = 0;
    let mut steps = 0;

    loop {
        if idx < 0 || idx >= len {
            break;
        }

        let val = jmplist[idx as usize];

        if weird_offsets && val >= 3 {
            jmplist[idx as usize] -= 1;
        } else {
            jmplist[idx as usize] += 1;
        }

        idx += val;
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
