use aoc25::{Input, load_input};
use itertools::Itertools;

const DAY: u8 = 6;

fn main() {
    let lines = load_input(DAY, Input::Puzzle);
    let a = solve_part_a(&lines);
    println!("Solution to a: {a}");
    let b = solve_part_b(&lines);
    println!("Solution to b: {b}");
}

fn solve_part_a(lines: &[String]) -> u64 {
    let mut line_iters: Vec<_> = lines.iter().map(|l| l.split_whitespace()).collect();
    let (operations, number_strings) = line_iters
        .split_last_mut()
        .expect("Couldn't unwarp list of line iters");
    let mut sum: u64 = 0;
    for op in operations {
        let numbers: Vec<u64> = number_strings
            .iter_mut()
            .map(|n| n.next().expect("").parse().unwrap())
            .collect();
        let total: u64 = match op {
            "*" => numbers.iter().product(),
            "+" => numbers.iter().sum(),
            _ => panic!("Unrecognised operand"),
        };
        sum += total;
    }
    sum
}

fn solve_part_b(lines: &[String]) -> u64 {
    let mut line_iters: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let (operations_line, number_strings) = line_iters
        .split_last_mut()
        .expect("Couldn't unwarp list of line iters");
    let mut sum: u64 = 0;
    let mut total = 0;
    let mut current_op = ' ';
    let max_length = number_strings
        .iter()
        .map(|n| n.len())
        .max()
        .expect("Couldn't find max");
    for i in 0..max_length {
        let op_line_char = operations_line.get(i);
        let vertical_number: u64 = number_strings
            .iter_mut()
            .filter_map(|n| n.get(i).and_then(|d| d.to_digit(10)))
            .fold(0, |accumulation, digit| accumulation * 10 + digit as u64);
        if vertical_number == 0 {
            continue;
        }
        if op_line_char == Some(&'*') || op_line_char == Some(&'+') {
            sum += total;
            total = vertical_number;
            current_op = *op_line_char.unwrap();
        } else {
            match current_op {
                '*' => total = total * vertical_number,
                '+' => total = total + vertical_number,
                _ => panic!("Unrecognised operand {op_line_char:?}"),
            }
        };
    }
    sum += total;
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(4277556, solve_part_a(&load_input(DAY, Input::Test)));
        assert_eq!(6503327062445, solve_part_a(&load_input(DAY, Input::Puzzle)));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(3263827, solve_part_b(&load_input(DAY, Input::Test)));
        // assert_eq!(6503327062445, solve_part_b(&load_input(DAY, Input::Puzzle)));
    }
}
