#[allow(dead_code)]
const INPUT: &str = "input";
#[allow(dead_code)]
const TEST_INPUT: &str = "test_input";

fn solution_part1(file_path: &str) -> u64 {
    let file = std::fs::read_to_string(file_path).expect("Cannot open file");
    let mut lines = file.lines();
    let nums: Vec<Vec<u64>> = (0..lines.clone().count() - 1)
        .map(|line_idx| {
            lines
                .next()
                .unwrap_or_else(|| panic!("Missing number line {}", line_idx + 1))
                .split_whitespace()
                .map(|s| {
                    s.parse::<u64>()
                        .unwrap_or_else(|_| panic!("Invalid number: {}", s))
                })
                .collect()
        })
        .collect();
    let operators: Vec<char> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|c| c.chars().next().unwrap())
        .collect();

    (0..operators.len())
        .map(|col_idx| {
            let mut iter = nums.iter().map(|row| row[col_idx]);
            let first = iter.next().expect("Column cannot be empty");
            let op = operators[col_idx];
            iter.fold(first, |acc, x| match op {
                '*' => acc * x,
                '+' => acc + x,
                _ => panic!("invalid op {}", op),
            })
        })
        .sum()
}

fn solution_part2(file_path: &str) -> u64 {
    // read columns from right to left
    // read push parse column into a number and push into a buffer
    // do that until an operator ('+', '*') is found, then skip one column

    let mut data: Vec<Vec<char>> = std::fs::read_to_string(file_path)
        .expect("Cannot open file")
        .lines()
        .map(|line| {
            // reverse during parsing instead during processing
            line.chars().rev().collect()
        })
        .collect();

    let op_line = data.pop().expect("File was empty");
    let line_len = op_line.len();
    assert!(
        data.iter().all(|v| v.len() == line_len),
        "Lines are not the same length!"
    );

    let mut sum = 0u64;
    let mut num_buffer: Vec<u64> = Vec::new();
    for col in 0..line_len {
        let Ok(celaphod_num) = data
            .iter()
            .map(|line| line[col])
            .collect::<String>()
            .trim()
            .parse::<u64>()
        else {
            num_buffer.clear();
            continue;
        };
        num_buffer.push(celaphod_num);

        match op_line[col] {
            '*' => sum += num_buffer.iter().product::<u64>(),
            '+' => sum += num_buffer.iter().sum::<u64>(),
            _ => {},
        };
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(4277556, solution_part1(TEST_INPUT));
        assert_eq!(4412382293768, solution_part1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3263827, solution_part2(TEST_INPUT));
        assert_eq!(7858808482092, solution_part2(INPUT));
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
