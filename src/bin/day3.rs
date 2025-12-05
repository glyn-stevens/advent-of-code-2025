use anyhow::{Context, Result};
use aoc25::{Input, load_input};
use itertools::Itertools;

fn main() {
    let lines = load_input(3, Input::Puzzle);
    let a = solve_part_a(&lines);
    println!("Solution to a: {a}");
    let b = solve_part_b(&lines);
    println!("Solution to b: {b}");
}

fn sum_largest_n_digit_numbers(lines: &[String], n: u32) -> Result<u64> {
    lines
        .iter()
        .map(|line| largest_n_digit_num(&parse_line(&line)?, n))
        .sum()
}

fn solve_part_a(lines: &[String]) -> u64 {
    sum_largest_n_digit_numbers(lines, 2).expect("Failed to solve")
}

fn solve_part_b(lines: &[String]) -> u64 {
    sum_largest_n_digit_numbers(lines, 12).expect("Failed to solve")
}

fn parse_line(line: &str) -> Result<Vec<u32>> {
    line.chars()
        .map(|c| c.to_digit(10).context("Failed to convert to digit"))
        .collect()
}

fn largest_n_digit_num(input: &[u32], n: u32) -> Result<u64> {
    if n < 2 {
        input
            .iter()
            .max()
            .context("Input is empty")
            .map(|&val| val as u64)
    } else {
        // We can't pick any of the last n-1 chars as the number must be n digits
        let search_space_end = input.len() + 1 - n as usize;

        let reversed_idx = &input[..search_space_end]
            .iter()
            .rev() // `position_max` gives position of last item if several are equally maximum
            .position_max()
            .context("Couldn't find max position")?;

        // Convert reversed position to original index
        let max_digit_idx = search_space_end - 1 - reversed_idx;

        let next_input = &input[max_digit_idx + 1..];

        Ok(input[max_digit_idx] as u64 * 10_u64.pow(n - 1)
            + largest_n_digit_num(&next_input, n - 1)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part_a() {
        assert_eq!(357, solve_part_a(&load_input(3, Input::Test)));
        assert_eq!(17074, solve_part_a(&load_input(3, Input::Puzzle)));
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_largest_n_digit_num(#[case] input: String, #[case] expected: u64) {
        assert_eq!(
            largest_n_digit_num(&parse_line(&input).unwrap(), 12).unwrap(),
            expected
        )
    }

    #[test]
    fn test_part_b() {
        assert_eq!(3121910778619, solve_part_b(&load_input(3, Input::Test)));
        assert_eq!(169512729575727, solve_part_b(&load_input(3, Input::Puzzle)));
    }
}
