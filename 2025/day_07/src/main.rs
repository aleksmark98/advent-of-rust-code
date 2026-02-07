use bit_vec::BitVec;

#[allow(dead_code)]
const INPUT: &str = "input";
#[allow(dead_code)]
const TEST_INPUT: &str = "test_input";

fn solution_part1(file_path: &str) -> u64 {
    let file = std::fs::read_to_string(file_path).expect("Cannot open file");
    let mut lines = file.lines();

    let first_line = lines.next().expect("File was empty");
    let linelength = first_line.len();

    let mut beams = BitVec::from_elem(linelength, false);
    let start_idx = first_line
        .find('S')
        .expect("First line is missing the starting \'S\' character");
    beams.set(start_idx, true);

    let mut split_counter: u64 = 0;
    for row in lines {
        for (splitter, _) in row.match_indices('^') {
            if beams[splitter] {
                split_counter += 1;
                beams.set(splitter, false);
                beams.set(std::cmp::max(0, splitter - 1), true);
                beams.set(std::cmp::min(splitter + 1, linelength - 1), true);
            }
        }
    }

    split_counter
}

fn solution_part2(file_path: &str) -> u64 {
    let file = std::fs::read_to_string(file_path).expect("Cannot open file");
    let mut lines = file.lines();

    let first_line = lines.next().expect("File was empty");
    let linelength = first_line.len();

    let mut timelines = vec![0u64; linelength];
    let start_idx = first_line
        .find('S')
        .expect("First line is missing the starting \'S\' character");
    timelines[start_idx] = 1;

    for row in lines {
        for (splitter, _) in row.match_indices('^') {
            let num_timelines = timelines[splitter];
            if num_timelines == 0 {
                continue;
            }
            timelines[splitter] = 0;
            timelines[std::cmp::max(0, splitter - 1)] += num_timelines;
            timelines[std::cmp::min(splitter + 1, linelength - 1)] += num_timelines;
        }
    }

    timelines.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(21, solution_part1(TEST_INPUT));
        assert_eq!(1518, solution_part1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(40, solution_part2(TEST_INPUT));
        assert_eq!(25489586715621, solution_part2(INPUT));
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

