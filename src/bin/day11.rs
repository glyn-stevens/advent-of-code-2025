use aoc25::{Input, load_input};
use std::collections::HashMap;

const DAY: u8 = 11;

#[derive(Debug, Clone, Copy, Default)]
struct RouteCombinations {
    // Number of routes to the exit node that have a 'dac' node in them, have an 'fft' node, have both, or neither.
    dac: u64,
    fft: u64,
    both: u64,
    neither: u64,
}

impl RouteCombinations {
    fn total(&self) -> u64 {
        self.dac + self.fft + self.both + self.neither
    }

    fn combine(&self, other: &RouteCombinations) -> RouteCombinations {
        RouteCombinations {
            dac: self.dac + other.dac,
            fft: self.fft + other.fft,
            both: self.both + other.both,
            neither: self.neither + other.neither,
        }
    }
}

fn main() {
    let lines = load_input(DAY, Input::Puzzle);
    let a = solve_part_a(&lines);
    println!("Solution to a: {a}");
    let b = solve_part_b(&lines);
    println!("Solution to b: {b}");
}

fn solve_part_a(lines: &[String]) -> u64 {
    let nodes: HashMap<String, Vec<String>> = lines.iter().map(parse_node).collect();
    let mut cache = HashMap::new();
    let start_node = "you";
    let exit_node = "out";
    route_combinations(&nodes, &start_node, &exit_node, &mut cache).total()
}

fn solve_part_b(lines: &[String]) -> u64 {
    let nodes: HashMap<String, Vec<String>> = lines.iter().map(parse_node).collect();
    let mut cache = HashMap::new();
    let start_node = "svr";
    let exit_node = "out";
    route_combinations(&nodes, &start_node, &exit_node, &mut cache).both
}

fn route_combinations(
    all_nodes: &HashMap<String, Vec<String>>,
    current_node: &str,
    exit_node: &str,
    cache: &mut HashMap<String, RouteCombinations>,
) -> RouteCombinations {
    let mut routes = RouteCombinations::default();
    for next_node in all_nodes
        .get(current_node)
        .expect("Couldn't find current node")
        .iter()
    {
        if next_node == exit_node {
            routes.neither += 1;
        } else if let Some(solutions_from_next_node) = cache.get(next_node.as_str()) {
            routes = routes.combine(&update_routes_with_node(
                current_node,
                *solutions_from_next_node,
            ));
        } else {
            let routes_from = route_combinations(all_nodes, next_node, exit_node, cache);
            routes = routes.combine(&update_routes_with_node(&current_node, routes_from));
        }
    }
    cache.insert(current_node.to_string(), routes);
    routes
}

fn update_routes_with_node(node_to_add: &str, routes: RouteCombinations) -> RouteCombinations {
    match node_to_add {
        "dac" => RouteCombinations {
            dac: routes.neither + routes.dac,
            fft: 0,
            both: routes.fft + routes.both,
            neither: 0,
        },
        "fft" => RouteCombinations {
            dac: 0,
            fft: routes.neither + routes.fft,
            both: routes.dac + routes.both,
            neither: 0,
        },
        _ => routes,
    }
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

    #[test]
    fn test_part_b() {
        assert_eq!(2, solve_part_b(&load_input(DAY, Input::TestB)));
        assert_eq!(
            533996779677200,
            solve_part_b(&load_input(DAY, Input::Puzzle))
        );
    }
}
