// #![feature(test)]
// extern crate test;

use itertools::{Itertools, izip};

#[allow(dead_code)]
const INPUT: &str = "input";
#[allow(dead_code)]
const TEST_INPUT: &str = "test_input";

struct Point {
    x: u32,
    y: u32,
}
struct Rectangle {
    x_min: u32,
    x_max: u32,
    y_min: u32,
    y_max: u32,
}
impl Rectangle {
    fn new(a: &Point, b: &Point) -> Self {
        Rectangle {
            x_min: a.x.min(b.x),
            x_max: a.x.max(b.x),
            y_min: a.y.min(b.y),
            y_max: a.y.max(b.y),
        }
    }
}

fn read_polygon(file_path: &str) -> Vec<Point> {
    let file = std::fs::read_to_string(file_path).expect("Cannot open file");

    let mut points: Vec<Point> = Vec::with_capacity(file.lines().count());
    for point in file.lines() {
        let mut parts = point.split(',');
        points.push(Point {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
        });
    }

    points
}

fn is_pt_on_edge(pt: &Point, edge_pt_a: &Point, edge_pt_b: &Point) -> bool {
    let (a, b) = (edge_pt_a, edge_pt_b);

    // (pt.x == a.x && pt.x == b.x && a.y.min(b.y) <= pt.y && pt.y <= a.x.max(b.x))
    // || (pt.x == a.x && pt.x == b.x && a.x.min(b.x) <= pt.x && pt.x <= a.x.max(b.x))

    let edge_is_horizontal = a.x == b.x;
    let edge_is_vertical = a.y == b.y;
    std::debug_assert!(edge_is_vertical != edge_is_horizontal);


    if edge_is_vertical {
        let (x_min, x_max) = (a.x.min(b.x), a.x.max(b.x));
        pt.x == a.x && (x_min..=x_max).contains(&pt.x)
    } else {
        let (y_min, y_max) = (a.y.min(b.y), a.y.max(b.y));
        pt.x == a.x && (y_min..=y_max).contains(&pt.y)
    }
}

fn edge_intersects_rect(a: &Point, b: &Point, r: &Rectangle) -> bool {
    if a.y == b.y {
        if r.y_min < a.y && a.y < r.y_max && a.x.max(b.x) > r.x_min && r.x_max > a.x.min(b.x) {
            return true;
        }
    } else {
        #[allow(clippy::collapsible_else_if)]
        if r.x_min < a.x && a.x < r.x_max && a.y.max(b.y) > r.y_min && r.y_max > a.y.min(b.y) {
            return true;
        }
    }

    false
}

fn is_rectangle_valid(pt1: &Point, pt2: &Point, polygon: &Vec<Point>) -> bool {
    let rect = Rectangle::new(pt1, pt2);
    let rect_points_to_check = [Point { x: pt1.x, y: pt2.y }, Point { x: pt2.x, y: pt1.y }];
    let mut are_rect_points_inside = [false; 2];
    let mut keep_checking_rect_points = [true; 2];

    for (a, b) in polygon.iter().circular_tuple_windows() {
        for (pt, inside, check_pt) in izip!(
            &rect_points_to_check,
            &mut are_rect_points_inside,
            &mut keep_checking_rect_points
        ) {
            // check if rect points are inside the polygon
            if *check_pt && is_pt_on_edge(pt, a, b) {
                *inside = true;
                *check_pt = false;
            }

            if *check_pt {
                let pt_in_vertical_span = (a.y > pt.y) != (b.y > pt.y);
                let boundary_cross_check = if pt_in_vertical_span && a.y != b.y {
                    let (px, py, ax, ay, bx, by) = (
                        pt.x as i64,
                        pt.y as i64,
                        a.x as i64,
                        a.y as i64,
                        b.x as i64,
                        b.y as i64,
                    );
                    px < (ax - bx) * (py - ay) / (by - ay) + ax
                } else {
                    false
                };
                if boundary_cross_check {
                    *inside = !*inside;
                }
            }
        }
        if edge_intersects_rect(a, b, &rect) {
            return false;
        }
    }
    if !are_rect_points_inside.into_iter().any(|b| b) {
        return false;
    }

    true
}

fn solution_part1(polygon: &Vec<Point>) -> usize {
    polygon
        .iter()
        .tuple_combinations()
        .fold(0, |max_area, (pt_a, pt_b)| {
            let (width, heigth) = (pt_a.x.abs_diff(pt_b.x) + 1, pt_a.y.abs_diff(pt_b.y) + 1);
            let area = width as usize * heigth as usize;
            max_area.max(area)
        })
}

fn solution_part2(polygon: &Vec<Point>) -> usize {
    polygon
        .iter()
        .tuple_combinations()
        .fold(0, |old_area, (pt_a, pt_b)| {
            let (width, heigth) = (pt_a.x.abs_diff(pt_b.x) + 1, pt_a.y.abs_diff(pt_b.y) + 1);
            let new_area = width as usize * heigth as usize;

            if new_area > old_area && is_rectangle_valid(pt_a, pt_b, &polygon) {
                new_area
            } else {
                old_area
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_square_valid() {
        let polygon = read_polygon(TEST_INPUT);

        assert!(is_rectangle_valid(
            &Point { x: 9, y: 5 },
            &Point { x: 2, y: 3 },
            &polygon
        ));
        assert!(is_rectangle_valid(
            &Point { x: 7, y: 1 },
            &Point { x: 11, y: 1 },
            &polygon
        ));
    }

    #[test]
    fn test_part_1() {
        let polygon = read_polygon(TEST_INPUT);
        assert_eq!(50, solution_part1(&polygon));
        let polygon = read_polygon(INPUT);
        assert_eq!(4750297200, solution_part1(&polygon));
    }

    #[test]
    fn test_part_2() {
        let polygon = read_polygon(TEST_INPUT);
        assert_eq!(24, solution_part2(&polygon));
        let polygon = read_polygon(INPUT);
        assert_eq!(1578115935, solution_part2(&polygon));
    }

    // #[bench]
    // fn bench_add_two(b: &mut test::Bencher) {
    //     let polygon = read_polygon(INPUT);
    //
    //     b.iter(|| solution_part2(&polygon));
    // }
}

fn main() {
    let file_path = TEST_INPUT;
    // let file_path = INPUT;
    let polygon = read_polygon(file_path);

    println!("\"{file_path}\" part 1: {}", solution_part1(&polygon));
    println!("\"{file_path}\" part 2: {}", solution_part2(&polygon));
}
