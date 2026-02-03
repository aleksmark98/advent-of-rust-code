use std::fs;
const INPUT: &str = "input";
const TEST_INPUT: &str = "test_input";

fn parse_input(file_path: &str) -> Vec<Vec<u16>> {
    fs::read_to_string(file_path)
        .expect("Cannot open file")
        .lines()
        .map(|line| {
            line.to_owned()
                .replace(".", "0")
                .replace("@", "1")
                .chars()
                .map(|c| c.to_digit(10).expect("Should only contain digits") as u16)
                .collect()
        })
        .collect()
}

fn box_accum_3x3(input: &Vec<Vec<u16>>) -> Vec<Vec<u16>> {
    let height = input.len();
    let width = input[0].len();

    let mut tmp = vec![vec![0u16; width]; height];

    // ---- Horizontal pass (sum of 3 neighbors) ----
    for y in 0..height {
        for x in 0..width {
            let mut sum = input[y][x];

            if x > 0 {
                sum += input[y][x - 1];
            }
            if x + 1 < width {
                sum += input[y][x + 1];
            }

            tmp[y][x] = sum;
        }
    }

    // ---- Vertical pass (sum of 3 neighbors) ----
    let mut out = vec![vec![0u16; width]; height];

    for y in 0..height {
        for x in 0..width {
            let mut sum = tmp[y][x];

            if y > 0 {
                sum += tmp[y - 1][x];
            }
            if y + 1 < height {
                sum += tmp[y + 1][x];
            }

            out[y][x] = sum;
        }
    }

    out
}

fn solution_part1(file_path: &str) -> u64 {
    let rolls = parse_input(file_path);
    let mut box_acc = box_accum_3x3(&rolls);

    let mut freed_rolls = 0u64;
    for y in 0..rolls.len() {
        for x in 0..rolls[0].len() {
            if rolls[y][x] == 1 {
                box_acc[y][x] -= 1;
                if box_acc[y][x] < 4 {
                    freed_rolls += 1;
                }
            }
        }
    }
    freed_rolls
}
 
fn count_neighbor_rolls(rolls: &mut Vec<Vec<u16>>) -> u64 {
    let mut neighbor_counts = box_accum_3x3(&rolls);

    let mut freed_rolls = 0u64;
    for (roll_row, neighbors_row) in rolls.iter_mut().zip(neighbor_counts.iter_mut()) {
        for (roll, neighbor_count) in roll_row.iter_mut().zip(neighbors_row.iter_mut()) {
            if *roll == 1 {
                *neighbor_count -= 1;
                if *neighbor_count < 4 {
                    freed_rolls += 1;
                    // `rolls` mutation is the only difference from part 1
                    *roll = 0;
                }
            }
        }
    }
    freed_rolls
}


fn solution_part2(file_path: &str) -> u64 {
    let mut rolls = parse_input(file_path);

    let mut extractable_rolls = 0u64;
    let max_iters = 1000;
    for _ in 0..max_iters {
        let freed_rolls = count_neighbor_rolls(&mut rolls);
        extractable_rolls += freed_rolls;

        if freed_rolls == 0 {
            return extractable_rolls
        }
    }
    extractable_rolls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(13, solution_part1(TEST_INPUT));
        assert_eq!(1486, solution_part1(INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(43, solution_part2(TEST_INPUT));
        assert_eq!(9024, solution_part2(INPUT));
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
