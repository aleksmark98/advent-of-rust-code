use binarray::BinaryArray;
use std::collections::{HashSet, VecDeque};
use rayon::prelude::*;
use z3::{Optimize, SatResult, ast::Int};

#[allow(dead_code)]
const INPUT: &str = "input";
#[allow(dead_code)]
const TEST_INPUT: &str = "test_input";

pub struct Machine {
    target: u16,
    buttons: Vec<u16>,
    joltage: Vec<usize>,
}

impl Machine {
    fn convert_buttons(&self) -> Vec<Vec<usize>> {
        self.buttons.iter().map(|button| button.to_indices()).collect()
    }

    fn fewest_presses_joltage(&self) -> usize {
        let buttons = self.convert_buttons();
        let opt = Optimize::new();
        let total = Int::fresh_const("total");

        let presses: Vec<Int> = (0..buttons.len())
            .map(|idx| Int::fresh_const(&format!("button_{idx}")))
            .collect();

        presses.iter().for_each(|button| opt.assert(&button.ge(0)));

        for (pos, &target) in self.joltage.iter().enumerate() {
            let sum = Int::add(
                &buttons
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, button)| {
                        button.contains(&pos).then(||&presses[idx])
                    })
                    .collect::<Vec<&Int>>(),
            );
            opt.assert(&sum.eq(Int::from_u64(target as u64)));
        }

        opt.assert(&total.eq(Int::add(&presses)));
        opt.minimize(&total);

        match opt.check(&[]) {
            SatResult::Sat => opt
                .get_model()
                .unwrap()
                .eval(&total, true)
                .and_then(|t| t.as_u64())
                .unwrap() as usize,
            _              => panic!("No solution found"),
        }
    }

    fn fewest_presses_lights(&self) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        while let Some((state, n)) = queue.pop_front() {
            if visited.contains(&state) {
                continue;
            }
            if state == self.target {
                return n;
            }
            visited.insert(state.clone());
            for button in self.buttons.iter() {
                let next = state ^ button;
                queue.push_back((next, n + 1));
            }
        }

        unreachable!()
    }
}

pub fn input_generator(file_path: &str) -> Vec<Machine> {
    let input = std::fs::read_to_string(file_path).expect("Cannot open file");

    input.lines().map(|line| {
        let mut parts: Vec<&str> = line.split_whitespace().collect();
        let first = parts.remove(0);

        // Machine variables
        let mut target = 0_u16;
        let mut buttons = Vec::new();
        let mut joltage = Vec::new();

        for (idx, ch) in first[1..first.len()-1].chars().enumerate() {
            if ch == '#' {
                target.set_bit(idx, true);
            }
        }

        for part in parts {
            // Split up each capsule
            let cap = part.chars().next().unwrap(); 
            let interior = &part[1..part.len()-1];
            let values: Vec<usize> = interior.split(',').map(|num| num.parse::<usize>().unwrap()).collect();
            match cap {
                '(' => {
                    let mut button = 0_u16;
                    for index in values {
                        button.set_bit(index, true);
                    }
                    buttons.push(button);
                },
                '{' => joltage = values,
                _   => panic!("Unexpected cap."),
            }
        }

        Machine { target, buttons, joltage }
    })
    .collect()
}

fn solution<F>(process_machine: F, input: &Vec<Machine>) -> usize 
    where F: Fn(&Machine) -> usize + Sync + Send,
{
    input
        .par_iter()
        .map(process_machine)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            solution(Machine::fewest_presses_lights, &input_generator(TEST_INPUT)),
            7
        );
        assert_eq!(
            solution(Machine::fewest_presses_lights, &input_generator(INPUT)),
            558
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            solution(
                Machine::fewest_presses_joltage,
                &input_generator(TEST_INPUT)
            ),
            33
        );
        assert_eq!(
            solution(Machine::fewest_presses_joltage, &input_generator(INPUT)),
            20317
        );
    }
}

fn main() {
    // let file_path = TEST_INPUT;
    let file_path = INPUT;

    println!(
        "\"{file_path}\" part 1: {}",
        solution(Machine::fewest_presses_lights, &input_generator(file_path))
    );
    println!(
        "\"{file_path}\" part 2: {}",
        solution(Machine::fewest_presses_joltage, &input_generator(file_path))
    );
}
