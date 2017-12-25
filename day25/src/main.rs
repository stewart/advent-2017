use std::collections::HashSet;

const NUM_STEPS: usize = 12317297;

#[derive(Debug)]
enum State { A, B, C, D, E, F }

fn main() {
    use State::*;

    let mut tape: HashSet<isize> = HashSet::new();
    let mut cursor = 0;
    let mut state = A;

    for _ in 0..NUM_STEPS {
        match state {
            A => {
                if tape.contains(&cursor) {
                    tape.remove(&cursor);
                    cursor -= 1;
                    state = D;
                } else {
                    tape.insert(cursor);
                    cursor += 1;
                    state = B;
                }
            }

            B => {
                if tape.contains(&cursor) {
                    tape.remove(&cursor);
                    cursor += 1;
                    state = F;
                } else {
                    tape.insert(cursor);
                    cursor += 1;
                    state = C
                }
            }

            C => {
                if tape.contains(&cursor) {
                    tape.insert(cursor);
                    cursor -= 1;
                    state = A
                } else {
                    tape.insert(cursor);
                    cursor -= 1;
                    state = C
                }
            }

            D => {
                if tape.contains(&cursor) {
                    tape.insert(cursor);
                    cursor += 1;
                    state = A
                } else {
                    tape.remove(&cursor);
                    cursor -= 1;
                    state = E
                }
            }

            E => {
                if tape.contains(&cursor) {
                    tape.remove(&cursor);
                    cursor += 1;
                    state = B;
                } else {
                    tape.insert(cursor);
                    cursor -= 1;
                    state = A;
                }
            }

            F => {
                if tape.contains(&cursor) {
                    tape.remove(&cursor);
                    cursor += 1;
                    state = E;
                } else {
                    tape.remove(&cursor);
                    cursor += 1;
                    state = C;
                }
            }
        }
    }

    println!("1 -> {}", tape.len());
}
