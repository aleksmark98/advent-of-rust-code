// #![feature(test)]
// extern crate test;

use std::collections::HashMap;
use pathfinding::directed::count_paths::count_paths;

const INPUT: &str = "input";

fn build_graph(input: String) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line|{
            let (start, outs) = line.split_once(':').expect("invalid line format");
            let outs: Vec<String> = outs.split(' ').map(String::from).collect();

            (start.to_string(), outs)
        })
        .collect()
} 

fn solution_part_1(file_path: &str) -> usize 
{
    let input = std::fs::read_to_string(file_path).expect("Cannot open file");
    let graph = build_graph(input);

    count_paths(
        "you".to_string(),
        |node: &String| graph.get(node).into_iter().flatten().cloned(),
        |node| *node == "out",
    )
             
}

fn solution_part_2(file_path: &str) -> usize 
{
    let input = std::fs::read_to_string(file_path).expect("Cannot open file");
    let graph = build_graph(input);

    // number of solutions is finite, so graph must be a acyclic
    // this implies that paths between "dac" and "fft" can be only in one direction,
    // meaning either dac_fft is zero or fft_dac is zero

    let dac_fft = count_paths(
        "dac".to_string(),
        |node: &String| graph.get(node).into_iter().flatten().cloned(),
        |node| *node == "fft",
    );
    if dac_fft != 0 {
        let svr_dac = count_paths(
            "svr".to_string(),
            |node: &String| graph.get(node).into_iter().flatten().cloned(),
            |node| *node == "dac",
        );
        let fft_out = count_paths(
            "fft".to_string(),
            |node: &String| graph.get(node).into_iter().flatten().cloned(),
            |node| *node == "out",
        );
        svr_dac * dac_fft * fft_out
    } else {
        let svr_fft = count_paths(
            "svr".to_string(),
            |node: &String| graph.get(node).into_iter().flatten().cloned(),
            |node| *node == "fft",
        );
        let fft_dac = count_paths(
            "fft".to_string(),
            |node: &String| graph.get(node).into_iter().flatten().cloned(),
            |node| *node == "dac",
        );
        let dac_out = count_paths(
            "dac".to_string(),
            |node: &String| graph.get(node).into_iter().flatten().cloned(),
            |node| *node == "out",
        );
        svr_fft * fft_dac * dac_out
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!( solution_part_1("test_input_1"), 5);
        assert_eq!( solution_part_1(INPUT), 670);
    }

    #[test]
    fn test_part_2() {
        assert_eq!( solution_part_2("test_input_2"), 2);
        assert_eq!( solution_part_2(INPUT), 332052564714990);
    }

    // #[bench]
    // fn bench_part_2(b: &mut test::Bencher) {
    //
    //     b.iter(|| solution_part_2(INPUT));
    // }
    // #[bench]
    // fn bench_creating_graph(b: &mut test::Bencher) {
    //
    //     b.iter(|| {
    //         let input = std::fs::read_to_string(INPUT).expect("Cannot open file");
    //         build_graph(input)
    //     });
    // }
}

fn main() {
    println!("part 1: {}", solution_part_1(INPUT));
    println!("part 2: {}", solution_part_2(INPUT));
}
