use std::fs;
const INPUT: &str = "input";
const TEST_INPUT: &str = "test_input";

fn max_joltage_twopasses_part1(bank: &str) -> u64 {
    let (start_pos, first_digit) = bank[..bank.len() - 1]
        .char_indices()
        .filter_map(|(pos, c)| c.to_digit(10).map(|digit| (pos, digit)))
        .max_by_key(|&(pos, digit)| (digit, std::cmp::Reverse(pos)))
        .unwrap_or((0, 0));

    let second_digit = bank[start_pos + 1..]
        .chars()
        .filter_map(|c| c.to_digit(10))
        .max()
        .unwrap_or(0);

    (first_digit * 10 + second_digit) as u64
}

fn solution_part1(file_path: &str) -> u64 {
    fs::read_to_string(file_path)
        .expect("Cannot open file")
        .lines()
        .map(|bank| max_joltage_twopasses_part1(bank))
        .sum()
}

fn max_joltage_part2(bank: &str) -> u64 {
    const NUM_BATTERIES: usize = 12;

    let digits: Vec<u32> = bank
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    let mut start = 0;

    (0..NUM_BATTERIES)
        .rev()
        .fold(0u64, |joltage, end_pad| {
            let (idx, &digit) = digits[start..digits.len() - end_pad]
                .iter()
                .enumerate()
                .max_by_key(|&(i, &d)| (d, std::cmp::Reverse(i)))
                .unwrap();

            start += idx + 1;
            joltage + u64::from(digit) * 10u64.pow(end_pad as u32)
        })
}

fn solution_part2(file_path: &str) -> u64 {
    fs::read_to_string(file_path)
        .expect("Cannot open file")
        .lines()
        .map(|bank| max_joltage_part2(bank))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(357, solution_part1(TEST_INPUT));
        assert_eq!(17278, solution_part1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3121910778619, solution_part2(TEST_INPUT));
        assert_eq!(171528556468625, solution_part2(INPUT));
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
