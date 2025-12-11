puzzle_runner::register_chapter!(book = "2025", title = "Reactor");

use std::collections::{HashMap, VecDeque};

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_input(input: &str) -> Graph<'_> {
    parse!(input => {
        [graph split on '\n' into (Graph) with
            {
                node
                ": "
                [edges split]
            }
            => (node, edges)
        ]
    } => graph)
}

pub fn part1(input: &str) -> usize {
    let graph = parse_input(input);
    let mut queue = VecDeque::new();
    queue.push_back("you");
    let mut result = 0;
    while let Some(node) = queue.pop_front() {
        if node == "out" {
            result += 1;
            continue;
        }
        queue.extend(&graph[node]);
    }
    result
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 5)]
    static EXAMPLE_INPUT: &str = "
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
    ";
}
