use aoc25::{Input, load_input};
use itertools::Itertools;
use plotters::prelude::*;

const DAY: u8 = 9;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(PartialEq, Eq, Debug)]
enum CornerType {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug)]
struct Corner {
    pt: Point,
    compatible_as: Vec<CornerType>,
}

fn main() {
    let lines = load_input(DAY, Input::Puzzle);
    plot(&lines);
    let a = solve_part_a(&lines);
    println!("Solution to a: {a}");
    let b = solve_part_b(&lines);
    println!("Solution to b: {b}");
}

fn plot(lines: &[String]) {
    let root_area = BitMapBackend::new("images/day9_plot.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let points: Vec<_> = lines.iter().map(parse_line).collect();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Day 9 points", ("sans-serif", 40))
        .build_cartesian_2d(0..100_000, 0..100_000)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        points
            .iter()
            .map(|pt| TriangleMarker::new((pt.x as i32, pt.y as i32), 5, &BLUE)),
    )
    .unwrap();
}
fn solve_part_a(lines: &[String]) -> u64 {
    let all_points = lines.iter().map(parse_line).collect();
    sorted_sizes(&all_points)[0]
}

fn solve_part_b(lines: &[String]) -> u64 {
    let all_corner_points = lines.iter().map(parse_line).collect();
    let all_points = boundary_points(&all_corner_points, 100);
    let all_corners = to_corners(&all_corner_points);
    let sorted_candidates = sorted_sizes_corners(&all_corners);
    for (candidate_square, size) in sorted_candidates {
        if !all_points
            .iter()
            .any(|pt| square_contains(pt, &(candidate_square.0.pt, candidate_square.1.pt)))
        {
            return size;
        }
    }
    panic!("Couldn't find any solution to part b")
}

fn boundary_points(corner_points: &Vec<Point>, step_size: usize) -> Vec<Point> {
    corner_points
        .iter()
        .chain(corner_points.iter().take(1))
        .tuple_windows()
        .map(|points| points_between(points, step_size))
        .flatten()
        .unique()
        .collect()
}

fn points_between(points: (&Point, &Point), step_size: usize) -> Vec<Point> {
    let mut result: Vec<Point> = if points.0.x == points.1.x {
        let from = points.0.y.min(points.1.y);
        let to = points.0.y.max(points.1.y);
        (from..to)
            .step_by(step_size)
            .map(|y| Point { x: points.0.x, y })
            .collect()
    } else {
        let from = points.0.x.min(points.1.x);
        let to = points.0.x.max(points.1.x);
        (from..to)
            .step_by(step_size)
            .map(|x| Point { x, y: points.0.y })
            .collect()
    };

    // Add points.0 if it's not already first
    if result.first() != Some(points.0) {
        result.insert(0, *points.0);
    }

    result
}

fn to_corners(points: &Vec<Point>) -> Vec<Corner> {
    let mut padded_pts = points.clone();
    padded_pts.insert(0, *points.last().unwrap());
    padded_pts.push(points[0]);

    padded_pts
        .iter()
        .tuple_windows()
        .map(|(before, pt, after)| create_corner(before, pt, after))
        .collect()
}

fn create_corner(before: &Point, centre: &Point, after: &Point) -> Corner {
    Corner {
        pt: *centre,
        compatible_as: get_corner_type(before, centre, after),
    }
}
fn get_corner_type(before: &Point, centre: &Point, after: &Point) -> Vec<CornerType> {
    // Illustration of all 8 corner types. Corners going clockwise.
    // (x,y) = (0,0) is the top left corner.
    // ..###..
    // ..###..
    // ..###..
    // #######
    // #######
    // #######
    // ..###..
    // ..###..
    // ..###..
    if before.x == centre.x {
        // Vertical
        if before.y < centre.y {
            // Going up
            if after.x > centre.x {
                // Up -> Right
                return vec![CornerType::TopLeft];
            } else {
                // Up -> Left
                return vec![
                    CornerType::BottomRight,
                    CornerType::BottomLeft,
                    CornerType::TopLeft,
                ];
            }
        } else {
            // Going down
            if after.x > centre.x {
                // Down -> Right
                return vec![
                    CornerType::TopRight,
                    CornerType::TopLeft,
                    CornerType::BottomRight,
                ];
            } else {
                // Down -> Left
                return vec![CornerType::BottomRight];
            }
        }
    } else if before.y == centre.y {
        // Horizontal
        if before.x < centre.x {
            // Going right
            if after.y > centre.y {
                // Right -> Down
                return vec![CornerType::TopRight];
            } else {
                // Right -> Up
                return vec![
                    CornerType::TopRight,
                    CornerType::BottomLeft,
                    CornerType::TopLeft,
                ];
            }
        } else {
            // Going left
            if after.y > centre.y {
                // Left -> Down
                return vec![
                    CornerType::TopRight,
                    CornerType::BottomRight,
                    CornerType::BottomLeft,
                ];
            } else {
                // Left -> Up
                return vec![CornerType::BottomLeft];
            }
        }
    } else {
        panic!("Before {before:?} doesn't line up with centre {centre:?}")
    }
}

fn compatible_corners(a: &Corner, b: &Corner) -> bool {
    for type_a_corner in a.compatible_as.iter() {
        if b.compatible_as
            .iter()
            .contains(&opposite_corner_type(&type_a_corner))
        {
            // Check corner positions are consistent (e.g. Bottom Left can't be above Top Right)
            return match type_a_corner {
                CornerType::TopLeft => a.pt.x <= b.pt.x && a.pt.y <= b.pt.y,
                CornerType::BottomLeft => a.pt.x <= b.pt.x && a.pt.y >= b.pt.y,
                CornerType::BottomRight => a.pt.x >= b.pt.x && a.pt.y >= b.pt.y,
                CornerType::TopRight => a.pt.x >= b.pt.x && a.pt.y <= b.pt.y,
            };
        }
    }
    false
}

fn opposite_corner_type(corner: &CornerType) -> CornerType {
    match corner {
        CornerType::TopLeft => CornerType::BottomRight,
        CornerType::BottomLeft => CornerType::TopRight,
        CornerType::BottomRight => CornerType::TopLeft,
        CornerType::TopRight => CornerType::BottomLeft,
    }
}

fn square_contains(point: &Point, square: &(Point, Point)) -> bool {
    range_containts(point.x, (square.0.x, square.1.x))
        && range_containts(point.y, (square.0.y, square.1.y))
}

fn range_containts(test: u32, range: (u32, u32)) -> bool {
    let upper = range.0.max(range.1);
    let lower = range.0.min(range.1);
    test < upper && test > lower
}

fn sorted_sizes_corners(points: &Vec<Corner>) -> Vec<((&Corner, &Corner), u64)> {
    points
        .iter()
        .combinations(2)
        .map(|vec| ((vec[0], vec[1]), square_size(vec[0].pt, vec[1].pt)))
        .sorted_by(|(_, sizea), (_, sizeb)| sizeb.cmp(sizea))
        .filter(|((corner_a, corner_b), _)| compatible_corners(corner_a, corner_b))
        .collect()
}

fn sorted_sizes(points: &Vec<Point>) -> Vec<u64> {
    points
        .iter()
        .combinations(2)
        .map(|vec| square_size(*vec[0], *vec[1]))
        .sorted()
        .rev()
        .collect()
}

fn square_size(a: Point, b: Point) -> u64 {
    ((a.x as i64 - b.x as i64 + 1).abs() * (a.y as i64 - b.y as i64 + 1).abs()) as u64
}

fn parse_line(line: &String) -> Point {
    let parts: Vec<_> = line.split(",").collect();
    Point {
        x: parts[0].parse().expect("Couldn't parse"),
        y: parts[1].parse().expect("Couldn't parse"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(50, solve_part_a(&load_input(DAY, Input::Test)));
        assert_eq!(4749672288, solve_part_a(&load_input(DAY, Input::Puzzle)));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(24, solve_part_b(&load_input(DAY, Input::Test)));
        assert_eq!(1479665889, solve_part_b(&load_input(DAY, Input::Puzzle)));
    }
}
