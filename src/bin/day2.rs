use aoc25::{Input, load_input};
use itertools::Itertools;
use rstest::rstest;
fn main() {
    let lines = load_input(2, Input::Puzzle);
    let a = solve_part_a(&lines[0]);
    println!("Solution to a: {a}");
    let b = solve_part_b(&lines[0]);
    println!("Solution to b: {b}");
}

fn solve_part_a(line: &str) -> u64 {
    line.split(",")
        .inspect(|x| println!("For range: {x}"))
        .map(parse_range)
        .flat_map(|range| numbers_with_n_repeats(range, 2))
        .inspect(|x| println!("  - '{x}'"))
        .sum()
}

fn solve_part_b(line: &str) -> u64 {
    line.split(",")
        .inspect(|x| println!("For range {x}, found the following repeated numbers:"))
        .map(parse_range)
        .flat_map(numbers_with_any_repeats)
        .inspect(|x| println!("  - '{x}'"))
        .unique()
        .sum()
}

fn numbers_with_any_repeats(range: (u64, u64)) -> Vec<u64> {
    let max_repeats_possible = range.1.to_string().len();
    (2..max_repeats_possible + 1)
        .flat_map(|n| numbers_with_n_repeats(range, n as u32))
        .collect()
}

fn numbers_with_n_repeats(range: (u64, u64), n: u32) -> Vec<u64> {
    let next_number = |s| next_number_with_n_repeats(s, n);
    let mut next = next_number(range.0);
    let mut repeats = Vec::new();
    while next <= range.1 {
        repeats.push(next);
        next = next_number(next + 1);
    }
    repeats
}

fn next_number_with_n_repeats(start: u64, n: u32) -> u64 {
    let start_str = next_num_with_length_thats_multiple_of_n(start, n).to_string();
    let (first_part_str, _) = start_str.split_at(start_str.len() / n as usize);
    let first_part: u64 = first_part_str.parse().unwrap();
    if repeat_number(first_part, n) >= start {
        repeat_number(first_part, n)
    } else {
        repeat_number(first_part + 1, n)
    }
}

fn repeat_number(num_to_repeat: u64, n: u32) -> u64 {
    num_to_repeat
        .to_string()
        .repeat(n as usize)
        .parse::<u64>()
        .unwrap()
}

fn next_num_with_length_thats_multiple_of_n(start: u64, n: u32) -> u64 {
    let digits = start.to_string().len() as u32;
    if digits % n == 0 {
        start
    } else {
        10_u64.pow((digits / n + 1) * n - 1)
    }
}

fn parse_range(range_string: &str) -> (u64, u64) {
    let split_res: Vec<&str> = range_string.split("-").collect();
    let start = split_res[0];
    let end = split_res[1];
    let msg = "Couldn't parse range string";
    (start.parse().expect(msg), end.parse().expect(msg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(10, 11)]
    #[case(0, 11)]
    #[case(11, 11)]
    #[case(1000, 1010)]
    #[case(1012, 1111)]
    #[case(100, 1010)]
    fn test_next_number_with_2_repeats(#[case] input: u64, #[case] expected: u64) {
        assert_eq!(next_number_with_n_repeats(input, 2), expected);
    }

    #[rstest]
    #[case(100, 3, 111)]
    #[case(100000, 3, 101010)]
    #[case(100000, 6, 111111)]
    #[case(100000, 5, 1010101010)]
    #[case(2828255673, 5, 2828282828)]
    fn test_next_number_with_n_repeats(#[case] start: u64, #[case] n: u32, #[case] expected: u64) {
        assert_eq!(next_number_with_n_repeats(start, n), expected)
    }

    #[rstest]
    #[case(100, 2, 1000)]
    #[case(100, 2, 1000)]
    #[case(100, 3, 100)]
    #[case(1001, 3, 100_000)]
    #[case(10010, 3, 100_000)]
    #[case(10010, 4, 1000_0000)]
    #[case(1000_0000, 4, 1000_0000)]
    #[case(2828255673, 5, 2828255673)]
    fn test_correct_number_of_digits(#[case] start: u64, #[case] n: u32, #[case] expected: u64) {
        assert_eq!(next_num_with_length_thats_multiple_of_n(start, n), expected);
    }

    #[test]
    fn test_numbers_with_any_repeats() {
        assert_eq!(numbers_with_any_repeats((11, 22)), [11, 22])
    }

    #[test]
    fn test_part_a() {
        assert_eq!(1227775554, solve_part_a(&load_input(2, Input::Test)[0]));
        assert_eq!(38158151648, solve_part_a(&load_input(2, Input::Puzzle)[0]));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(4174379265, solve_part_b(&load_input(2, Input::Test)[0]));
        assert_eq!(45283684555, solve_part_b(&load_input(2, Input::Puzzle)[0]));
    }
}
