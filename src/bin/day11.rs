use aoc25::{Input, load_input};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

const DAY: u8 = 11;

struct State {}

fn main() {
    let lines = load_input(DAY, Input::Puzzle);
    let a = solve_part_a(&lines);
    println!("Solution to a: {a}");
}

fn solve_part_a(lines: &[String]) -> u32 {
    let nodes: HashMap<String, Vec<String>> = lines.iter().map(parse_node).collect();
    let mut solutions_from_node = HashMap::new();
    number_of_routes(&nodes, &"you".to_string(), &mut solutions_from_node)
}

fn number_of_routes(
    nodes: &HashMap<String, Vec<String>>,
    position: &String,
    solutions_from_node: &mut HashMap<String, u32>,
) -> u32 {
    let mut total = 0u32;
    for next_position in nodes[position].iter() {
        if next_position == "out" {
            total += 1;
        } else if let Some(x) = solutions_from_node.get(next_position) {
            total += *x;
        } else {
            let result = number_of_routes(nodes, next_position, solutions_from_node);
            total += result;
            solutions_from_node.insert(next_position.clone(), result);
        }
    }
    solutions_from_node.insert(position.clone(), total);
    total
}

fn parse_node(line: &String) -> (String, Vec<String>) {
    let (id, edges) = line.split_once(':').expect("Couldn't split line at :");
    (
        id.to_string(),
        edges.split_whitespace().map(|s| s.to_string()).collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(5, solve_part_a(&load_input(DAY, Input::Test)));
        assert_eq!(683, solve_part_a(&load_input(DAY, Input::Puzzle)));
    }
}
