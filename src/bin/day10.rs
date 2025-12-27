use aoc25::{Input, load_input};
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Formatter;
use std::time::Instant;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

const DAY: u8 = 10;

#[derive(Eq, PartialEq, Clone)]
struct Edge {
    states_toggled: Vec<u8>,
}

impl std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.states_toggled)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Graph {
    target_state: Vec<bool>,
    edges: Vec<Edge>,
}

#[derive(Debug, Eq, PartialEq)]
struct GraphPartB {
    target_state: Vec<u16>,
    edges: Vec<Edge>,
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    state: Vec<bool>,
    cost: u32,
}

#[derive(Debug, Eq, PartialEq)]
struct NodePartB {
    state: Vec<u16>,
    cost: u32,
    edges: Vec<Edge>,
}

impl Ord for NodePartB {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
        // .then_with(||)
    }
}

impl PartialOrd for NodePartB {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
        // .then_with(||)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let lines = load_input(DAY, Input::Puzzle);
    let a = solve_part_a(&lines);
    println!("Solution to a: {a}");
    let b = solve_part_b(&lines);
    println!("Solution to b: {b}");
}

fn solve_part_a(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(parse_graph)
        .map(|graph| solve_graph(&graph, vec![false; graph.target_state.len()]))
        .sum()
}

fn solve_part_b(lines: &[String]) -> u32 {
    lines
        .iter()
        .inspect(|line| println!("Solving {line}"))
        .map(parse_graph_part_b)
        .map(|graph| solve_graph_part_b(&graph, vec![0; graph.target_state.len()]))
        .sum()
}

fn generate_safe_compositions(
    target_needed: u32,
    edges: &[Edge],
    current_state: &[u16],
    target_state: &[u16],
) -> Vec<Vec<u32>> {
    if edges.is_empty() {
        return vec![];
    }

    if edges.len() == 1 {
        // Special case: only one edge, just return the target needed
        return vec![vec![target_needed]];
    }

    // Calculate maximum safe traversals for each edge
    // An edge can be traversed at most min(remaining) times across all nodes it affects
    let max_safe: Vec<u32> = edges
        .iter()
        .map(|edge| {
            edge.states_toggled
                .iter()
                .map(|&idx| {
                    let remaining =
                        target_state[idx as usize].saturating_sub(current_state[idx as usize]);
                    remaining as u32
                })
                .min()
                .unwrap_or(0)
        })
        .collect();

    let mut results = Vec::new();
    let mut current = vec![0u32; edges.len()];

    fn backtrack(
        results: &mut Vec<Vec<u32>>,
        current: &mut Vec<u32>,
        position: usize,
        remaining: u32,
        max_safe: &[u32],
    ) {
        if position == current.len() - 1 {
            // Last position gets all remaining, but check it doesn't exceed max safe
            if remaining <= max_safe[position] {
                current[position] = remaining;
                results.push(current.clone());
            }
            return;
        }

        // Try all possible values for current position, up to max_safe limit
        let max_here = remaining.min(max_safe[position]);
        for value in 0..=max_here {
            current[position] = value;
            backtrack(results, current, position + 1, remaining - value, max_safe);
        }
    }

    backtrack(&mut results, &mut current, 0, target_needed, &max_safe);
    results
}

fn solve_graph_part_b(graph: &GraphPartB, initial_state: Vec<u16>) -> u32 {
    let start_time = Instant::now();
    let mut states_visited = HashSet::with_capacity(40_000_000);
    states_visited.insert(initial_state.clone());

    let mut queue: BinaryHeap<_> = BinaryHeap::with_capacity(40_000_000);
    let mut current_node = NodePartB {
        state: initial_state,
        cost: 0,
        edges: graph.edges.clone(),
    };
    let mut cheapest: u32 = graph.target_state.iter().map(|&x| x as u32).sum();
    let mut iteration = 0;
    loop {
        iteration += 1;
        if iteration % 20000 == 0 {
            let elapsed = start_time.elapsed();
            println!(
                "Iteration {iteration}. Time: {:.2}s. Queue size: {}. Cheapest: {cheapest}. Current node: edges: {:?}, cost: {:?}, state: {:?}",
                elapsed.as_secs_f64(),
                queue.len(),
                current_node.edges,
                current_node.cost,
                current_node.state
            )
        }
        if current_node.cost >= cheapest {
            if queue.is_empty() {
                break;
            }
            current_node = queue.pop().unwrap();
            continue;
        }
        let unsatisfied_nodes: Vec<u8> = current_node
            .state
            .iter()
            .enumerate()
            .filter(|(idx, val)| val < &&graph.target_state[*idx])
            .map(|(idx, _)| idx as u8)
            .collect();

        if !unsatisfied_nodes.is_empty() {
            let node_edge_counts: HashMap<u8, usize> = unsatisfied_nodes
                .iter()
                .map(|&node| {
                    let count = current_node
                        .edges
                        .iter()
                        .filter(|edge| edge.states_toggled.contains(&node))
                        .count();
                    (node, count)
                })
                .collect();

            let maybe_node_with_fewest_edges = node_edge_counts
                .iter()
                .min_by_key(|&(&node, &count)| {
                    let remaining =
                        graph.target_state[node as usize] - current_node.state[node as usize];
                    (count, remaining)
                })
                .map(|(&node, _)| node);

            if let Some(node_with_fewest_edges) = maybe_node_with_fewest_edges {
                let (edges_to_loop_this_time, remaining_edges_new): (Vec<_>, Vec<_>) = current_node
                    .edges
                    .into_iter()
                    .partition(|edge| edge.states_toggled.contains(&node_with_fewest_edges));
                if !edges_to_loop_this_time.is_empty() {
                    let target_for_this_node = graph.target_state[node_with_fewest_edges as usize];
                    let current_value = current_node.state[node_with_fewest_edges as usize];
                    let traverses_needed = target_for_this_node - current_value;
                    let compositions = generate_safe_compositions(
                        traverses_needed as u32,
                        &edges_to_loop_this_time,
                        &current_node.state,
                        &graph.target_state,
                    );

                    for composition in compositions {
                        let mut next_state = current_node.state.clone();
                        let cost_increment: u32 = composition.iter().sum();

                        for (i, &traverses) in composition.iter().enumerate() {
                            if traverses > 0 {
                                next_state = calculate_state_part_b(
                                    &next_state,
                                    &edges_to_loop_this_time[i].states_toggled,
                                    traverses as u16,
                                );
                            }
                        }
                        if !has_overshot(&next_state, &graph.target_state)
                            && !states_visited.contains(&next_state)
                        {
                            if next_state == graph.target_state {
                                cheapest = cheapest.min(current_node.cost + cost_increment)
                            } else {
                                states_visited.insert(next_state.clone());
                                queue.push(NodePartB {
                                    state: next_state.clone(),
                                    cost: current_node.cost + cost_increment,
                                    edges: remaining_edges_new.clone(),
                                });
                            }
                        }
                    }
                }
            } else {
                println!("No unsatisfied node with edges found!");
                panic!("Algorithm error");
            }
        }
        if queue.is_empty() {
            break;
        }
        current_node = queue.pop().unwrap();
    }

    println!("\n=== SOLUTION FOUND ===");
    println!("Final cost: {}", cheapest);
    cheapest
}

fn has_overshot(states: &Vec<u16>, targets: &Vec<u16>) -> bool {
    states
        .iter()
        .zip(targets)
        .any(|(state, target)| state > target)
}

fn solve_graph(graph: &Graph, initial_state: Vec<bool>) -> u32 {
    let mut states_visited = Vec::from([initial_state.clone()]);
    let mut queue: BinaryHeap<_> = BinaryHeap::new();
    let mut current_node = Node {
        state: initial_state,
        cost: 0,
    };
    // Cost can only increase by 1 each loop, and we're always exploring from the current cheapest,
    // so once we've found any solution it must be the cheapest
    while current_node.state != graph.target_state {
        for edge in &graph.edges {
            let next_state = calculate_state(&current_node.state, &edge.states_toggled);
            if !states_visited.contains(&next_state) {
                queue.push(Node {
                    state: next_state.clone(),
                    cost: current_node.cost + 1,
                });
                states_visited.push(next_state);
            }
        }
        current_node = queue.pop().expect("No more items in queue")
    }
    current_node.cost
}

fn calculate_state_part_b(
    current_state: &Vec<u16>,
    states_incremented: &Vec<u8>,
    traverses: u16,
) -> Vec<u16> {
    current_state
        .clone()
        .iter()
        .enumerate()
        .map(|(idx, val)| {
            if states_incremented.contains(&(idx as u8)) {
                val + traverses
            } else {
                *val
            }
        })
        .collect()
}

fn calculate_state(current_state: &Vec<bool>, states_toggled: &Vec<u8>) -> Vec<bool> {
    let mut new_state = current_state.clone();
    for &idx in states_toggled {
        new_state[idx as usize] = !current_state[idx as usize]
    }
    new_state
}

fn target_regex() -> Regex {
    Regex::new(r"\[([.|#]+)]").unwrap()
}

fn edges_regex() -> Regex {
    Regex::new(r"\(([\d,]+)\)").unwrap()
}

fn target_part_b_regex() -> Regex {
    Regex::new(r"\{([\d,]+)}").unwrap()
}

fn parse_graph_part_b(line: &String) -> GraphPartB {
    let edges: Vec<Edge> = edges_regex()
        .captures_iter(line)
        .map(|match_| parse_edge(match_.get(1).expect("No capture in match").as_str()))
        .collect();

    let target_state_string = target_part_b_regex()
        .captures(line)
        .expect(&format!(
            "Couldn't parse target from {line} - no matches found"
        ))
        .get(1)
        .unwrap()
        .as_str();

    GraphPartB {
        target_state: parse_separated_list_numbers_u16(target_state_string, ','),
        edges,
    }
}

fn parse_graph(line: &String) -> Graph {
    let target_state = target_regex()
        .captures(line)
        .expect(&format!(
            "Couldn't parse target from {line} - no matches found"
        ))
        .get(1)
        .unwrap()
        .as_str()
        .chars()
        .map(parse_state)
        .collect();

    let edges: Vec<Edge> = edges_regex()
        .captures_iter(line)
        .map(|match_| parse_edge(match_.get(1).expect("No capture in match").as_str()))
        .collect();

    Graph {
        target_state,
        edges,
    }
}

fn parse_edge(edge: &str) -> Edge {
    Edge {
        states_toggled: parse_separated_list_numbers_u8(&edge, ','),
    }
}

fn parse_separated_list_numbers_u8(edge: &str, separator: char) -> Vec<u8> {
    edge.split(separator)
        .map(|d| {
            d.parse()
                .expect(&format!("Couldn't parse digit {d} from edge {edge}"))
        })
        .collect()
}

fn parse_separated_list_numbers_u16(edge: &str, separator: char) -> Vec<u16> {
    edge.split(separator)
        .map(|d| {
            d.parse()
                .expect(&format!("Couldn't parse digit {d} from edge {edge}"))
        })
        .collect()
}

fn parse_state(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => panic!("Unknown state {c}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT_LINE: &str = "[.##.] (3) (1,3) {3,5,4,7}";
    #[test]
    fn test_target_regex() {
        let output = target_regex()
            .captures(EXAMPLE_INPUT_LINE)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        assert_eq!(output, ".##.");
    }

    #[test]
    fn test_parse_graph() {
        let expected = Graph {
            target_state: vec![false, true, true, false],
            edges: vec![
                Edge {
                    states_toggled: vec![3],
                },
                Edge {
                    states_toggled: vec![1, 3],
                },
            ],
        };
        assert_eq!(parse_graph(&EXAMPLE_INPUT_LINE.to_string()), expected);
    }

    // #[test]
    // fn test_part_a() {
    //     assert_eq!(7, solve_part_a(&load_input(DAY, Input::Test)));
    //     assert_eq!(488, solve_part_a(&load_input(DAY, Input::Puzzle)));
    // }

    // const GRAPH_A: &str = "[..###.####] (0,1,3,7,9) (1,2,4,7,8) (0,1,2,5,7) (1,3,4,5,7,8,9) (0,1,5,6,7,8,9) (0,1,3,4,6,8,9) (1,8) (3,4,9) (0,1,4,5,6) (2,4,7) (2,3,5,6,7,8,9) (0,1,2,4,5,6,9) {27,44,21,21,34,36,30,28,30,32}";
    //
    // #[test]
    // fn test_solve_graph() {
    //     let graph = parse_graph(&GRAPH_A.to_string());
    //     assert_eq!(7, solve_graph(&graph, vec![false; graph.target_state.len()]))
    // }

    #[test]
    fn test_part_b() {
        assert_eq!(33, solve_part_b(&load_input(DAY, Input::Test)));
    }
}
