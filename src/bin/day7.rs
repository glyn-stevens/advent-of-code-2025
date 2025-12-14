use aoc25::{Input, load_input};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const DAY: u8 = 7;

fn main() {
    let lines = load_input(DAY, Input::Puzzle);
    let a = solve_part_a(&lines);
    println!("Solution to a: {a}");
    let b = solve_part_b(&lines);
    println!("Solution to b: {b}");
}

fn solve_part_a(lines: &[String]) -> u64 {
    solve_for_beam_count_and_split_count(lines).1
}

fn solve_part_b(lines: &[String]) -> u64 {
    solve_for_beam_count_and_split_count(lines).0.values().sum()
}

fn solve_for_beam_count_and_split_count(lines: &[String]) -> (HashMap<usize, u64>, u64) {
    let mut beam_count_by_index = HashMap::new();
    beam_count_by_index.insert(start_index(&lines[0]), 1);
    let mut beam_split_count = 0;
    for line in lines[1..].iter() {
        let beam_props =
            calculate_beam_splitting(&beam_count_by_index, find_splitter_indices(line));
        beam_count_by_index = beam_props.0;
        beam_split_count += beam_props.1;
    }
    (beam_count_by_index, beam_split_count)
}

fn calculate_beam_splitting(
    beam_count_by_index: &HashMap<usize, u64>,
    splitter_indices: HashSet<usize>,
) -> (HashMap<usize, u64>, u64) {
    let mut new_beam_count_by_index = HashMap::new();
    let mut beam_split_count = 0;
    for (beam_idx, beam_count) in beam_count_by_index {
        if splitter_indices.contains(&beam_idx) {
            beam_split_count += 1;
            add_or_insert(beam_idx - 1, *beam_count, &mut new_beam_count_by_index);
            add_or_insert(beam_idx + 1, *beam_count, &mut new_beam_count_by_index);
        } else {
            add_or_insert(*beam_idx, *beam_count, &mut new_beam_count_by_index);
        }
    }
    (new_beam_count_by_index, beam_split_count)
}

fn add_or_insert(index: usize, value: u64, map: &mut HashMap<usize, u64>) {
    map.entry(index)
        .and_modify(|existing| *existing += value)
        .or_insert(value);
}

fn start_index(line: &str) -> usize {
    line.find('S').expect("Couldn't find start index")
}

fn find_splitter_indices(line: &str) -> HashSet<usize> {
    line.chars()
        .enumerate()
        .filter_map(|(idx, c)| is_splitter(&c).then_some(idx))
        .collect()
}

fn is_splitter(c: &char) -> bool {
    match c {
        '^' => true,
        '.' => false,
        _ => panic!("Unexpected char {c}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(21, solve_part_a(&load_input(DAY, Input::Test)));
        assert_eq!(1656, solve_part_a(&load_input(DAY, Input::Puzzle)));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(40, solve_part_b(&load_input(DAY, Input::Test)));
        assert_eq!(
            76624086587804,
            solve_part_b(&load_input(DAY, Input::Puzzle))
        );
    }
}
