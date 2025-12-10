use aoc25::{Input, load_input};
use itertools::any;
use std::slice::Iter;
const DAY: u8 = 5;

fn main() {
    let lines = load_input(DAY, Input::Puzzle);
    let a = solve_part_a(&lines);
    println!("Solution to a: {a}");
    let b = solve_part_b(&lines);
    println!("Solution to b: {b}");
}

fn solve_part_a(lines: &[String]) -> i32 {
    let (ranges, ids) = parse_input(lines.iter());
    let mut count = 0;
    for id in ids {
        if any(&ranges, |range| id >= range.0 && id <= range.1) {
            count += 1;
        }
    }
    count
}

fn solve_part_b(lines: &[String]) -> u64 {
    let (ranges, _) = parse_input(lines.iter());
    let mut count = 0;
    for range in combine_ranges(ranges) {
        count += range.1 - range.0 + 1;
    }
    count
}

fn combine_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort();
    let mut combined_ranges = Vec::new();
    for range in sorted_ranges {
        if contained_in_any(range, &combined_ranges) {
            continue;
        }
        let mut new_upper = range.1;
        loop {
            let overlapping_range_with_highest_upper_limit = ranges
                .iter()
                .filter(|r| r.0 <= new_upper && r.1 > new_upper)
                .max_by(|s, c| s.1.cmp(&c.1)); // Max upper range value

            if let Some(potential_new_upper) = overlapping_range_with_highest_upper_limit {
                // An overlapping range has a higher upper value. Use it.
                new_upper = new_upper.max(potential_new_upper.1);
            } else {
                combined_ranges.push((range.0, new_upper));
                break;
            }
        }
    }
    combined_ranges
}

fn contained_in_any(comparison: (u64, u64), ranges: &Vec<(u64, u64)>) -> bool {
    any(ranges, |range| {
        comparison.0 >= range.0 && comparison.1 <= range.1
    })
}

fn parse_input(lines: Iter<String>) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut finished_ranges = false;
    for line in lines {
        if line.len() == 0 as usize {
            finished_ranges = true;
            continue;
        }
        if finished_ranges {
            ids.push(line.parse().expect("Couldn't parse id line"));
        } else {
            ranges.push(parse_range(line));
        }
    }
    (ranges, ids)
}

fn parse_range(line: &str) -> (u64, u64) {
    let (lower, upper) = line.split_once('-').expect("Couldn't split range line");
    let msg = "Couldn't parse range";
    (
        lower.to_string().parse().expect(msg),
        upper.to_string().parse().expect(msg),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(3, solve_part_a(&load_input(DAY, Input::Test)));
        assert_eq!(558, solve_part_a(&load_input(DAY, Input::Puzzle)));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(14, solve_part_b(&load_input(DAY, Input::Test)));
        assert_eq!(
            344813017450467,
            solve_part_b(&load_input(DAY, Input::Puzzle))
        );
    }
}
