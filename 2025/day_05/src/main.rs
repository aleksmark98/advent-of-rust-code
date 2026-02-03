use std::cmp;
use std::fs;

const INPUT: &str = "input";
const TEST_INPUT: &str = "test_input";

fn merge_intervals(mut id_ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    id_ranges.sort_by_key(|(start, _)| *start);

    id_ranges
        .into_iter()
        .fold(Vec::new(), |mut result, (start, end)| {
            match result.last_mut() {
                Some((_, last_end)) if *last_end >= start => {
                    *last_end = cmp::max(*last_end, end);
                }
                _ => result.push((start, end)),
            }
            result
        })
}

fn parse_puzzle_input(file_path: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let file = fs::read_to_string(file_path).expect("Cannot open file");
    let (id_ranges_block, ids_block) = file
        .split_once("\n\n")
        .expect("input must contain two sections");

    let id_ranges = merge_intervals(
        id_ranges_block
            .lines()
            .filter_map(|line| line.split_once('-'))
            .map(
                |(start, end)| match (start.parse::<u64>(), end.parse::<u64>()) {
                    (Ok(s), Ok(e)) => (s, e),
                    _ => panic!("{start}, {end}"),
                },
            )
            .collect(),
    );

    let ids: Vec<u64> = ids_block
        .lines()
        .filter_map(|line| line.parse::<u64>().ok())
        .collect();

    (id_ranges, ids)
}

fn solution_part1(file_path: &str) -> u64 {
    let (id_ranges, ids) = parse_puzzle_input(file_path);

    ids.into_iter()
        .filter(|id| {
            id_ranges
                .iter()
                .any(|(start, end)| (start..=end).contains(&id))
        })
        .count() as u64
}

fn solution_part2(file_path: &str) -> u64 {
    let (id_ranges, _) = parse_puzzle_input(file_path);

    id_ranges
        .into_iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merging_intervals() {
        let input: Vec<(u64, u64)> = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let truth: Vec<(u64, u64)> = vec![(3, 5), (10, 20)];
        assert_eq!(truth, merge_intervals(input))
    }

    #[test]
    fn test_part_1() {
        assert_eq!(3, solution_part1(TEST_INPUT));
        assert_eq!(770, solution_part1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(14, solution_part2(TEST_INPUT));
        assert_eq!(357674099117260, solution_part2(INPUT));
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
