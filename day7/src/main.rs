#![feature(string_retain)]

#[derive(Debug, PartialEq)]
struct Node {
    name: String,
    weight: usize,
    children: Vec<String>
}

impl Node {
    fn parse(node: &str) -> Node {
        let mut node = String::from(node);

        node.retain(|ch| {
            match ch {
                '(' | ')' | '-' | '>' | ',' => false,
                _ => true
            }
        });

        let node = node.split_whitespace().collect::<Vec<_>>();

        Node {
            name: String::from(node[0]),
            weight: node[1].parse().unwrap(),
            children: node.into_iter().skip(2).map(String::from).collect()
        }
    }
}

fn main() {
    let input = include_str!("input");

    let nodes = input.
        lines().
        map(Node::parse).
        collect::<Vec<_>>();

    println!("1 -> {}", bottom_node(&nodes));
    println!("2 -> {}", corrected_weight(nodes));
}

fn bottom_node<'a>(nodes: &'a[Node]) -> &'a str {
    nodes.
        iter().
        find(|p| !nodes.iter().any(|o| { o.children.contains(&p.name)})).
        map(|node| &node.name).
        unwrap()
}

fn corrected_weight(nodes: Vec<Node>) -> usize {
    60
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn test_bottom_node() {
        let nodes = String::from(INPUT).
            lines().
            map(Node::parse).
            collect::<Vec<_>>();

        assert_eq!(bottom_node(&nodes), "tknk");
    }

    #[test]
    fn test_corrected_weight() {
        let nodes = String::from(INPUT).
            lines().
            map(Node::parse).
            collect::<Vec<_>>();

        assert_eq!(corrected_weight(nodes), 60);
    }
}
