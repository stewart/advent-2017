use Heading::*;

enum Heading {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let track: Vec<Vec<char>> = include_str!("input").
        lines().
        map(|line| line.chars().collect()).
        collect();

    let mut position = (0, 169);
    let mut steps = 0;
    let mut heading = Down;
    let mut letters: Vec<char> = Vec::new();

    loop {
        let (x, y) = position;
        let cur = track[x][y];

        match cur {
            cur @ 'A' ... 'Z' => { letters.push(cur); }

            '+' => {
                heading = match heading {
                    Up | Down => {
                        let left_valid = y > 0 && track[x][y - 1] == '-';
                        let right_valid = y < track[x].len() - 1 && track[x][y + 1] == '-';

                        if left_valid {
                            Left
                        } else if right_valid {
                            Right
                        } else {
                            panic!("Can't pick a direction: {:?}", position);
                        }
                    },

                    Left | Right => {
                        let up_valid = x > 0 && track[x - 1][y] == '|';
                        let down_valid = x < track.len() - 1 && track[x + 1][y] == '|';

                        if up_valid {
                            Up
                        } else if down_valid {
                            Down
                        } else {
                            panic!("Can't pick a direction: {:?}", position);
                        }
                    }
                }
            }

            ' ' => { break; }

            _ => {}
        }

        position = match heading {
            Up    => (x - 1, y),
            Down  => (x + 1, y),
            Left  => (x, y - 1),
            Right => (x, y + 1),
        };

        steps += 1;
    }

    println!("1 -> {}", letters.iter().collect::<String>());
    println!("2 -> {}", steps);
}
