use aoc25::{Input, load_input};
use std::collections::HashMap;

const DAY: u8 = 4;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum State {
    Filled,
    Empty,
    Taken,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

fn main() {
    let lines = load_input(DAY, Input::Puzzle);
    let a = solve_part_a(&lines);
    println!("Solution to a: {a}");
    let b = solve_part_b(&lines);
    println!("Solution to b: {b}");
}

fn solve_part_a(lines: &[String]) -> usize {
    let grid: Vec<Vec<State>> = lines.iter().map(|l| parse_line(&l)).collect();
    filled_location_with_num_filled_neighbours(&grid)
        .iter()
        .filter(|c| *c.1 < 4)
        .count()
}

fn solve_part_b(lines: &[String]) -> usize {
    let print = false;
    let mut grid: Vec<Vec<State>> = lines.iter().map(|l| parse_line(&l)).collect();
    let size = size(&grid);
    let mut filled_locations = filled_location_with_num_filled_neighbours(&grid);
    let initial_filled = filled_locations.len();
    loop {
        let removables: Vec<Coord> = filled_locations
            .iter()
            .filter(|(_, filled_neighbours)| **filled_neighbours < 4)
            .map(|(coord, _)| *coord)
            .collect();
        if removables.len() == 0 {
            break;
        }
        for removable in removables {
            filled_locations.remove(&removable);
            for neighbour in neighbours(&removable, &size) {
                // Update the count of neighbouring cells
                if let Some(count) = filled_locations.get(&neighbour) {
                    filled_locations.insert(neighbour, count - 1);
                }
            }
            if print {
                set_state_at(removable, State::Taken, &mut grid);
            }
        }
    }
    if print {
        for line in grid {
            let line_str: String = line.iter().map(state_to_char).collect();
            println!("{line_str}")
        }
    }
    initial_filled - filled_locations.len()
}

fn filled_location_with_num_filled_neighbours(grid: &Vec<Vec<State>>) -> HashMap<Coord, usize> {
    let size = size(grid);
    (0..size.y)
        .flat_map(|y| (0..size.x).map(move |x| Coord { x, y }))
        .filter(|c| state_at(c, &grid) == State::Filled)
        .map(|c| {
            (
                c,
                states_of_neighbour_coords(&c, &grid)
                    .iter()
                    .filter(|&&s| s == State::Filled)
                    .count(),
            )
        })
        .collect()
}

fn state_to_char(s: &State) -> char {
    match s {
        State::Taken => 'x',
        State::Filled => '@',
        State::Empty => '.',
    }
}

fn state_at(coord: &Coord, grid: &Vec<Vec<State>>) -> State {
    grid[coord.y as usize][coord.x as usize]
}

fn set_state_at(coord: Coord, state: State, grid: &mut Vec<Vec<State>>) {
    grid[coord.y as usize][coord.x as usize] = state
}

fn states_of_neighbour_coords(loc: &Coord, grid: &Vec<Vec<State>>) -> Vec<State> {
    neighbours(loc, &size(grid))
        .iter()
        .map(|n| state_at(n, &grid))
        .collect()
}

fn size(grid: &Vec<Vec<State>>) -> Coord {
    Coord {
        x: grid[0].len() as i32,
        y: grid.len() as i32,
    }
}

fn neighbours(loc: &Coord, grid_size: &Coord) -> Vec<Coord> {
    let mut neighbours = Vec::new();
    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            let potential = Coord {
                x: loc.x + dx,
                y: loc.y + dy,
            };
            if 0 <= potential.x
                && potential.x < grid_size.x
                && 0 <= potential.y
                && potential.y < grid_size.y
                && potential != *loc
            {
                neighbours.push(potential);
            }
        }
    }
    neighbours
}

fn parse_line(line: &str) -> Vec<State> {
    line.chars().map(parse_location).collect()
}

fn parse_location(c: char) -> State {
    match c {
        '@' => State::Filled,
        '.' => State::Empty,
        _ => panic!("Unexpected character in grid"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbours() {
        let output = neighbours(&Coord { x: 1, y: 1 }, &Coord { x: 10, y: 10 });
        let expected = [
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 1 },
            Coord { x: 0, y: 2 },
            Coord { x: 1, y: 0 },
            Coord { x: 1, y: 2 },
            Coord { x: 2, y: 0 },
            Coord { x: 2, y: 1 },
            Coord { x: 2, y: 2 },
        ];
        assert_eq!(output, expected)
    }
    #[test]
    fn test_part_a() {
        assert_eq!(13, solve_part_a(&load_input(DAY, Input::Test)));
        assert_eq!(1587, solve_part_a(&load_input(DAY, Input::Puzzle)));
    }
    #[test]
    fn test_part_b() {
        assert_eq!(43, solve_part_b(&load_input(DAY, Input::Test)));
        assert_eq!(8946, solve_part_b(&load_input(DAY, Input::Puzzle)));
    }
}
