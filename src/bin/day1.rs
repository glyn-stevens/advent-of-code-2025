use aoc25::{Input, load_input};

fn main() {
    let lines = load_input(1, Input::Puzzle);
    let a = solve_part_a(&lines);
    println!("Solution to a: {a}");
    let b = solve_part_b(&lines);
    println!("Solution to b: {b}")
}

fn solve_part_a(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| decode_line(line).expect("Couldn't decode line"))
        .scan(50, |total, increment| {
            *total += increment;
            Some(*total)
        })
        .filter(|total| total % 100 == 0)
        .count()
}

fn solve_part_b(lines: &[String]) -> i32 {
    let mut counter = 0;
    let mut prev_total = 50;
    for line in lines {
        let increment = decode_line(&line).expect("Couldn't decode line");
        let current_total = prev_total + increment;
        if prev_total % 100 == 0 {
            // Special case: Dial moving off a multiple of 100
            counter += increment.abs() / 100
        } else if current_total % 100 == 0 {
            // Special case: Dial moving on to a multiple of 100
            counter += 1 + increment.abs() / 100
        } else {
            counter += centuries_crossed(prev_total, current_total);
        }
        prev_total = current_total;
    }
    counter
}

fn centuries_crossed(a: i32, b: i32) -> i32 {
    (a.div_euclid(100) - b.div_euclid(100)).abs()
}

fn decode_line(line: &str) -> Result<i32, String> {
    let (dir, val) = line.split_at(1);
    let multiplier = match dir {
        "R" => 1,
        "L" => -1,
        _ => return Err(format!("Unknown direction specifier {dir}")),
    };
    val.parse::<i32>()
        .map(|n| multiplier * n)
        .map_err(|_| format!("Cannot parse {val}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(3, solve_part_a(&load_input(1, Input::Test)));
        assert_eq!(1120, solve_part_a(&load_input(1, Input::Puzzle)));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(6, solve_part_b(&load_input(1, Input::Test)));
        assert_eq!(6554, solve_part_b(&load_input(1, Input::Puzzle)));
    }
}
