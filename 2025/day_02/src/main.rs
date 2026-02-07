use std::fs;

#[allow(dead_code)]
const INPUT: &str = "input";
#[allow(dead_code)]
const TEST_INPUT: &str = "test_input";

fn check_invalid_id_part1(id: u64) -> bool {
    let id_str = id.to_string();
    if id_str.len() % 2 == 1 {
        return false;
    }
    let midpoint = id_str.len() / 2;

    id_str[0..midpoint] == id_str[midpoint..]
}

fn check_invalid_id_part2(id: u64) -> bool {
    let id = id.to_string();
    'substr_len: for test_len in 1..(id.len() / 2 + 1) {
        if id.len() % test_len != 0 {
            continue;
        }

        let occurences = id.len() / test_len;
        for count in 1..occurences {
            let offset = count * test_len;
            if id[offset..offset + test_len] != id[..test_len] {
                continue 'substr_len;
            }
        }

        return true;
    }
    false
}

fn solution<F: Fn(u64) -> bool>(file_path: &str, invalid_id_checker: F) -> u64 {
    fs::read_to_string(file_path)
        .expect("Cannot open file")
        .split(',')
        .filter_map(|range| {
            let mut ids = range.trim().split('-');
            let start = ids.next()?.parse::<u64>().ok()?;
            let end = ids.next()?.parse::<u64>().ok()?;
            Some((start, end))
        })
        .flat_map(|(start, end)| start..=end)
        .filter(|&id| invalid_id_checker(id))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(1227775554, solution(TEST_INPUT, check_invalid_id_part1));
        assert_eq!(43952536386, solution(INPUT, check_invalid_id_part1));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(4174379265, solution(TEST_INPUT, check_invalid_id_part2));
        assert_eq!(54486209192, solution(INPUT, check_invalid_id_part2));
    }
}

fn main() {
    let file_path = TEST_INPUT;
    // let file_path = INPUT;

    println!(
        "The solution part 1 for \"{file_path}\" is {}",
        solution(file_path, check_invalid_id_part1)
    );
    println!(
        "The solution part 2 for \"{file_path}\" is {}",
        solution(file_path, check_invalid_id_part2)
    );
}
