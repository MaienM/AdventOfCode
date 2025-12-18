puzzle_runner::register_chapter!(title = "Reactor");

use std::collections::HashMap;

use memoize::memoize;

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

#[memoize(Ignore: graph, Map: from -> String)]
fn find_paths_from(graph: &Graph, from: &str) -> usize {
    if from == "out" {
        return 1;
    }
    graph[from]
        .iter()
        .map(|edge| find_paths_from(graph, edge))
        .sum()
}

#[register_part]
fn part1(input: &str) -> usize {
    let graph = parse_input(input);
    find_paths_from(&graph, "you")
}

#[memoize(Ignore: graph, Map: from -> String)]
fn find_paths_from_through(graph: &Graph, from: &str, mut flags: u8) -> usize {
    match from {
        "out" => return usize::from(flags == 3),
        "dac" => {
            flags |= 1;
        }
        "fft" => {
            flags |= 2;
        }
        _ => {}
    }
    graph[from]
        .iter()
        .map(|edge| find_paths_from_through(graph, edge, flags))
        .sum()
}

#[register_part]
fn part2(input: &str) -> usize {
    let graph = parse_input(input);
    find_paths_from_through(&graph, "svr", 0)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 5)]
    static EXAMPLE_INPUT_1: &str = "
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

    #[example_input(part2 = 2)]
    static EXAMPLE_INPUT_2: &str = "
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
    ";
}
