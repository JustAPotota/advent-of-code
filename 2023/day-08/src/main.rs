use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_test() {
        let input = include_str!("test_input.txt");

        let (output, time_taken) = common::time_fn(part1, input);
        println!("Got '{output}' in {time_taken:#?}");
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test_input.txt");

        let (output, time_taken) = common::time_fn(part2, input);
        println!("Got '{output}' in {time_taken:#?}");
    }
}

#[derive(Debug)]
struct Map {
    turn_left: Vec<bool>,
    nodes: HashMap<String, (String, String)>,
    start: String,
    end: String,
}

fn parse_line(line: &str) -> (String, (String, String)) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let key = parts[0].to_string();
    let left_path = parts[2].chars().filter(|c| c.is_alphabetic()).collect();
    let right_path = parts[3].chars().filter(|c| c.is_alphabetic()).collect();
    (key, (left_path, right_path))
}

fn parse_turn_lefts(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == 'L').collect()
}

impl Map {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let first_line = lines.next().unwrap();
        let turn_left = first_line.chars().map(|c| c == 'L').collect();
        lines.next();

        let node_lines: Vec<&str> = lines.collect();
        let (start, _) = parse_line(node_lines[0]);
        let (end, _) = parse_line(node_lines.last().unwrap());

        let nodes = node_lines
            .into_iter()
            .fold(HashMap::new(), |mut nodes, line| {
                let (key, (left_path, right_path)) = parse_line(line);

                nodes.insert(key, (left_path, right_path));

                nodes
            });

        Self {
            turn_left,
            nodes,
            start,
            end,
        }
    }
}

fn parse_input(input: &str) -> (Vec<bool>, Vec<(usize, usize)>) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let turn_left = parse_turn_lefts(first_line);
    lines.next();

    let nodes: Vec<(String, (String, String))> = lines.map(parse_line).collect();

    let name_to_index: HashMap<String, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, node)| (node.0.clone(), i))
        .collect();

    let nodes: Vec<(usize, usize)> = nodes
        .iter()
        .map(|(_, (left_node, right_node))| {
            (
                *name_to_index.get(left_node).unwrap(),
                *name_to_index.get(right_node).unwrap(),
            )
        })
        .collect();

    (turn_left, nodes)
}

fn part1(input: &str) -> u32 {
    let (directions, nodes) = parse_input(input);

    let mut steps = 0;
    let mut next_index = 0;
    let end_index = nodes.len() - 1;
    for turn_left in directions.into_iter().cycle() {
        steps += 1;
        let (left_index, right_index) = nodes[next_index];
        next_index = if turn_left { left_index } else { right_index };
        if next_index == end_index {
            break;
        }
    }

    steps
}

fn part2(input: &str) -> u32 {
    todo!()
}

fn main() {
    common::main(include_str!("input.txt"), part1, part2)
}
