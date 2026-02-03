use std::collections::HashSet;

const INPUT: &str = "input";
const TEST_INPUT: &str = "test_input";

struct Point {
    x: u32,
    y: u32,
    z: u32,
}

fn dist(a: &Point, b: &Point) -> u32 {
    ((a.x.abs_diff(b.x) as f32).powf(2.)
        + (a.y.abs_diff(b.y) as f32).powf(2.)
        + (a.z.abs_diff(b.z) as f32).powf(2.)) as u32
}

fn get_pairs_sorted_by_distance(file_path: &str) -> (Vec<(usize, usize)>, Vec<Point>) {
    let file = std::fs::read_to_string(file_path).expect("Cannot open file");
    let lines = file.lines();

    let mut points: Vec<Point> = Vec::with_capacity(lines.clone().count());
    for point in lines {
        let mut parts = point.split(',');
        points.push(Point {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        });
    }

    let num_pairs = points.len() * (points.len() - 1) / 2;
    let mut pairs = Vec::with_capacity(num_pairs);
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = dist(&points[i], &points[j]);
            pairs.push(((i, j), distance))
        }
    }

    pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let pairs: Vec<(usize, usize)> = pairs.into_iter().map(|(pair, _)| pair).collect();

    (pairs, points)
}

fn update_clusters(pt_a: usize, pt_b: usize, clusters: &mut Vec<HashSet<usize>>) {
    let mut intersection: Vec<usize> = Vec::new();
    for (idx, set) in clusters.iter().enumerate() {
        if set.contains(&pt_a) | set.contains(&pt_b) {
            intersection.push(idx);
        }
    }
    match intersection.len() {
        0 => clusters.push(HashSet::from([pt_a, pt_b])),
        1 => {
            clusters[intersection[0]].insert(pt_a);
            clusters[intersection[0]].insert(pt_b);
        }
        2 => {
            clusters[intersection[0]].insert(pt_a);
            clusters[intersection[0]].insert(pt_b);
            let other_cluster = clusters.remove(intersection[1]);
            clusters[intersection[0]].extend(other_cluster);
        }
        _ => panic!(
            "clusters are meant to be disjoint, a pair cant intersect more than 2 existing clusters"
        ),
    }
}

fn solution_part1(file_path: &str) -> usize {
    let (pairs, _) = get_pairs_sorted_by_distance(file_path);

    let mut clusters: Vec<HashSet<usize>> = vec![HashSet::from([pairs[0].0, pairs[0].1])];

    let n_closest_pairs = if file_path == INPUT { 1000 } else { 10 };
    // MISTAKE - instead of take(n_closest_pairs) it should be 1..n_closest_pairs
    for (pt_a, pt_b) in pairs.into_iter().take(n_closest_pairs) {
        update_clusters(pt_a, pt_b, &mut clusters);
    }

    let mut lengths: Vec<usize> = clusters.into_iter().map(|set| set.len()).collect();
    lengths.sort_unstable();
    lengths.into_iter().rev().take(3).product()
}

fn solution_part2(file_path: &str) -> usize {
    let (pairs, points) = get_pairs_sorted_by_distance(file_path);

    let mut clusters: Vec<HashSet<usize>> = vec![HashSet::from([pairs[0].0, pairs[0].1])];

    for (pt_a, pt_b) in pairs.into_iter() {
        update_clusters(pt_a, pt_b, &mut clusters);

        if clusters.len() == 1 && clusters[0].len() == points.len() {
            return (points[pt_a].x as usize) * (points[pt_b].x as usize);
        }
    }
    panic!("All clusters should have been merged");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(40, solution_part1(TEST_INPUT));
        assert_eq!(175500, solution_part1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(25272, solution_part2(TEST_INPUT));
        assert_eq!(6934702555, solution_part2(INPUT));
    }
}

fn main() {
    let file_path = TEST_INPUT;
    // let file_path = INPUT;

    println!(
        "The solution part 1 for \"{file_path}\" is {}",
        solution_part1(file_path)
    );
    println!(
        "The solution part 2 for \"{file_path}\" is {}",
        solution_part2(file_path)
    );
}
