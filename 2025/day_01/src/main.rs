#[allow(dead_code)]
const INPUT: &str = "input";
#[allow(dead_code)]
const TEST_INPUT: &str = "test_input";

fn process_lock_part1(direction: &str, turns: i32, lock_position: i32, count: u64) -> (i32, u64) {
    let new_sum = match direction {
        "L" => (lock_position - turns).rem_euclid(100),
        "R" => (lock_position + turns).rem_euclid(100),
        _ => unreachable!(),
    };

    (new_sum, count + u64::from(new_sum == 0))
}

fn process_lock_part2(direction: &str, turns: i32, lock_position: i32, count: u64) -> (i32, u64) {
    let lock_val = match direction {
        "L" => lock_position - turns,
        "R" => lock_position + turns,
        _ => unreachable!(),
    };

    let updated_count = match lock_val {
        ..0 => {
            let new_count = count + (lock_val.abs() / 100) as u64;
            if lock_position != 0 {
                new_count + 1
            } else {
                new_count
            }
        }
        0 => count + 1,
        1..100 => count,
        100.. => count + (lock_val / 100) as u64, // TODO: make sure second term is not negative
    };

    let lock_position = lock_val.rem_euclid(100);

    (lock_position, updated_count)
}

fn solution<F: Fn(&str, i32, i32, u64) -> (i32, u64)>(file_path: &str, process_lock: F) -> u64 {
    let (_, count) = std::fs::read_to_string(file_path)
        .expect("Cannot open file")
        .lines()
        .map(|line| {
            let (direction, turns) = line.split_at(1);
            let turns = turns.parse::<i32>().expect("Integer parsing failed");
            (direction, turns)
        })
        .fold((50i32, 0u64), |(lock_position, count), (direction, turns)| {
        process_lock(direction, turns, lock_position, count)
    });

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(3, solution(TEST_INPUT, process_lock_part1));
        assert_eq!(1018, solution(INPUT, process_lock_part1));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(6, solution(TEST_INPUT, process_lock_part2));
        assert_eq!(5815, solution(INPUT, process_lock_part2));
    }
}

fn main() {
    let file_path = TEST_INPUT;
    // let file_path = INPUT;

    println!(
        "The solution part 1 for \"{file_path}\" is {}",
        solution(file_path, process_lock_part1)
    );
    println!(
        "The solution part 2 for \"{file_path}\" is {}",
        solution(file_path, process_lock_part2)
    );
}
