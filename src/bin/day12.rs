use aoc25::{Input, load_input};
use itertools::Itertools;
use regex::Regex;

const DAY: u8 = 12;

#[derive(Debug)]
struct Piece {
    lines: Vec<String>,
    number_squares_filled: usize,
}

#[derive(Debug)]
struct Puzzle {
    size: (u32, u32),
    pieces_required: Vec<u32>,
}

fn main() {
    let lines = load_input(DAY, Input::Puzzle);
    let a = solve_part_a(&lines);
    println!("Solution to a: {a}");
}

fn solve_part_a(lines: &[String]) -> usize {
    let (pieces, puzzles) = parse_lines(lines);
    let possible = puzzles.iter().filter(|p| is_possible(p)).count();
    let impossible = puzzles.iter().filter(|p| is_impossible(p, &pieces)).count();
    if possible + impossible != puzzles.len() {
        panic!(
            "Some puzzles can't be ruled definitely possible or definitely impossible with current checks. Got {possible} possible and {impossible} impossible. Total puzzles: {}",
            puzzles.len()
        );
    }
    possible
}

fn is_possible(puzzle: &Puzzle) -> bool {
    puzzle.size.0 / 3 * puzzle.size.1 / 3 >= puzzle.pieces_required.iter().sum()
}

fn is_impossible(puzzle: &Puzzle, pieces: &Vec<Piece>) -> bool {
    let squares_to_fill = pieces
        .iter()
        .zip_eq(&puzzle.pieces_required)
        .map(|(piece, quant)| piece.number_squares_filled as u32 * quant)
        .sum();
    puzzle.size.0 * puzzle.size.1 < squares_to_fill
}

fn parse_lines(lines: &[String]) -> (Vec<Piece>, Vec<Puzzle>) {
    let piece_id_regex = Regex::new(r"\d:").unwrap();
    let puzzle_size_regex = Regex::new(r"\d+x\d+").unwrap();
    let mut lines_iter = lines.iter();
    let mut pieces = Vec::new();
    let mut puzzles = Vec::new();
    while let Some(line) = lines_iter.next() {
        if let Some(puzzle_size_str) = puzzle_size_regex.find(line) {
            let pieces_required: Vec<u32> = line
                .split_once(':')
                .expect("Couldn't split puzzle line at ':'")
                .1
                .split_whitespace()
                .map(|num| num.parse().expect("Couldn't parse number"))
                .collect();
            let size = puzzle_size_str
                .as_str()
                .split_once('x')
                .map(|(s1, s2)| (s1.parse::<u32>().unwrap(), s2.parse::<u32>().unwrap()))
                .expect("Couldn't split size string at 'x'");
            puzzles.push(Puzzle {
                pieces_required,
                size,
            })
        } else if piece_id_regex.is_match(line) {
            let piece_lines = vec![
                lines_iter.next().unwrap().clone(),
                lines_iter.next().unwrap().clone(),
                lines_iter.next().unwrap().clone(),
            ];
            let number_squares_filled = piece_lines
                .iter()
                .flat_map(|line| line.chars())
                .filter(|&c| c == '#')
                .count();
            pieces.push(Piece {
                lines: piece_lines,
                number_squares_filled,
            });
        }
    }
    (pieces, puzzles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(474, solve_part_a(&load_input(DAY, Input::Puzzle)));
    }
}
