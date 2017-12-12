use std::collections::{HashMap, HashSet, VecDeque};

type Programs = HashMap<usize, Vec<usize>>;

fn main() {
    let mut programs: Programs = HashMap::new();

    include_str!("input").
        trim().
        lines().
        map(parse).
        for_each(|(id, contacts)| { programs.entry(id).or_insert(contacts); });

    println!("1 -> {:?}", neighbours(&programs, 0));
}

fn parse(input: &str) -> (usize, Vec<usize>) {
    let mut result = input.split("<->").map(str::trim);

    let id = result.
        next().expect("Program ID").
        parse::<usize>().expect("Program ID");

    let contacts = result.
        next().expect("Contacts").
        split(",").
        map(str::trim).
        map(|i| i.parse::<usize>().unwrap()).
        collect::<Vec<usize>>();

    (id, contacts)
}

fn neighbours(programs: &Programs, id: usize) -> usize {
    let mut group = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(id);

    while let Some(id) = queue.pop_front() {
        if group.contains(&id) {
            continue;
        }

        group.insert(id);

        for entry in programs.get(&id).expect("Map entry") {
            queue.push_back(*entry);
        }
    }

    group.len()
}

#[cfg(test)]
mod tests {
    use super::*;
}
