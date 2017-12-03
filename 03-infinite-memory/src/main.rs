fn main() {
    let input = 347991;
    println!("1 -> {}", steps(input));
    println!("2 -> {}", larger_value(input));
}

fn steps(n: usize) -> isize {
    let mut x: isize = 0;
    let mut y: isize = 0;

    let mut maxx = 0;
    let mut maxy = 0;
    let mut minx = 0;
    let mut miny = 0;

    let mut directionx = 1;
    let mut directiony = 0;

    for _ in 2..n+1 {
        x += directionx;
        y += directiony;

        if x > maxx {
            directiony = -1;
            directionx = 0;
            maxx = x;
        } else if x < minx {
            directionx = 0;
            directiony = 1;
            minx = x;
        }

        if y > maxy {
            directionx = 1;
            directiony = 0;
            maxy = y;
        } else if y < miny {
            directiony = 0;
            directionx = -1;
            miny = y;
        }
    }

    x.abs() + y.abs()
}

fn larger_value(n: usize) -> usize {
    let mut grid = vec![vec![0; 1024]; 1024];

    let mut x = 512;
    let mut y = 512;

    let mut k = 1;

    grid[x][y] = 1;

    macro_rules! update_grid {
        () => {
            match fill(&mut grid, x, y) {
                r if r > n => { return r },
                r => { grid[x][y] = r }
            }
        }
    }

    loop {
        for _ in 0..k {
            y += 1;
            update_grid!();
        }

        for _ in 0..k {
            x -= 1;
            update_grid!();
        }

        k += 1;

        for _ in 0..k {
            y -= 1;
            update_grid!();
        }

        for _ in 0..k {
            x += 1;
            update_grid!();
        }

        k += 1;
    }
}

fn fill(grid: &mut Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    let permutations = [
        (x - 1, y - 1), (x - 1, y), (x - 1, y + 1),
        (x, y - 1), (x, y + 1),
        (x + 1, y - 1), (x + 1, y), (x + 1, y + 1),
    ];

    permutations.
        iter().
        map(|&(x, y)| grid[x][y]).
        sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps() {
        assert_eq!(steps(1), 0);
        assert_eq!(steps(12), 3);
        assert_eq!(steps(23), 2);
        assert_eq!(steps(1024), 31);
    }

    #[test]
    fn test_larger_value() {
        assert_eq!(larger_value(3), 4);
        assert_eq!(larger_value(9), 10);
        assert_eq!(larger_value(360), 362);
    }
}
