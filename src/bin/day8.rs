use aoc25::{Input, load_input};
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

const DAY: u8 = 8;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point3d {
    x: i64,
    y: i64,
    z: i64,
}

impl fmt::Debug for Point3d {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(PartialEq, Hash, Clone, Copy)]
struct PointPair {
    a: Point3d,
    b: Point3d,
}

impl fmt::Debug for PointPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "a: {:?}, b: {:?}", self.a, self.b)
    }
}

type Circuit = HashSet<Point3d>;

fn main() {
    let lines = load_input(DAY, Input::Puzzle);
    let a = solve_part_a(&lines, 1000);
    println!("Solution to a: {a}");
    let b = solve_part_b(&lines);
    println!("Solution to b: {b}");
}

fn solve_part_a(lines: &[String], num_connection_to_make: u32) -> usize {
    let mut all_pairs_iter = point_pairs_sorted_by_distance(lines).into_iter();
    let mut circuits: Vec<Circuit> = Vec::new();

    for _ in 0..num_connection_to_make {
        let next_connection = all_pairs_iter
            .next()
            .expect("No more connections available");
        circuits = resolve_circuits_with_new_connection(circuits, &next_connection);
    }
    let mut sizes: Vec<usize> = circuits.iter().map(|c| c.len()).sorted().collect();
    sizes.pop().unwrap_or(1) * sizes.pop().unwrap_or(1) * sizes.pop().unwrap_or(1)
}

fn solve_part_b(lines: &[String]) -> i64 {
    let total_points = lines.len();
    let mut all_pairs_iter = point_pairs_sorted_by_distance(lines).into_iter();
    let mut circuits: Vec<Circuit> = Vec::new();
    let mut final_connection = None;
    while circuits.first().is_none_or(|c| c.len() < total_points) {
        let next_connection = all_pairs_iter
            .next()
            .expect("No more connections available");
        circuits = resolve_circuits_with_new_connection(circuits, &next_connection);
        final_connection = Some(next_connection)
    }
    let connection = final_connection.expect("Didn't make any connnections");
    connection.a.x * connection.b.x
}

fn resolve_circuits_with_new_connection(
    circuits: Vec<Circuit>,
    next_connection: &PointPair,
) -> Vec<Circuit> {
    let mut new_circ = HashSet::from([next_connection.a, next_connection.b]);
    let mut next_circuits: Vec<Circuit> = Vec::new();
    for circuit in circuits {
        if either_point_in_circuit(&next_connection, &circuit) {
            new_circ.extend(circuit);
        } else {
            next_circuits.push(circuit)
        }
    }
    next_circuits.push(new_circ);
    next_circuits
}

fn point_pairs_sorted_by_distance(lines: &[String]) -> Vec<(PointPair)> {
    let coords: Vec<_> = lines.iter().map(parse_line).collect();

    let mut all_pairs: Vec<(PointPair, f64)> = Vec::new();
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let pair = PointPair {
                a: coords[i],
                b: coords[j],
            };
            let dist = distance_between(&coords[i], &coords[j]);
            all_pairs.push((pair, dist));
        }
    }

    all_pairs.sort_by(|a, b| a.1.total_cmp(&b.1));
    all_pairs.iter().map(|(ptp, _)| *ptp).collect()
}

fn either_point_in_circuit(pair: &PointPair, circuit: &Circuit) -> bool {
    circuit.contains(&pair.a) || circuit.contains(&pair.b)
}

fn distance_between(a: &Point3d, b: &Point3d) -> f64 {
    (((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)) as f64).sqrt()
}

fn parse_line(line: &String) -> Point3d {
    let coords: Vec<u32> = line
        .split(',')
        .map(|c| c.parse().expect("Couldn't parse char as number"))
        .collect();
    Point3d {
        x: coords[0] as i64,
        y: coords[1] as i64,
        z: coords[2] as i64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(40, solve_part_a(&load_input(DAY, Input::Test), 10));
        assert_eq!(69192, solve_part_a(&load_input(DAY, Input::Puzzle), 1000));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(25272, solve_part_b(&load_input(DAY, Input::Test)));
        // assert_eq!(69192, solve_part_b(&load_input(DAY, Input::Puzzle)));
    }
}
