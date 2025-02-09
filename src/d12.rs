use crate::d11::expansion;
use std::fs;

fn parse_file(file_path: &str) -> Vec<(String, Vec<usize>)> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut maps = Vec::new();
    for line in file.lines() {
        let split: Vec<&str> = line.split(" ").collect();

        maps.push((
            split[0].to_string(),
            split[1].split(",").map(|c| c.parse().unwrap()).collect(),
        ));
    }
    maps
}

fn check_line(map: String, wanted_blocks: &Vec<usize>) -> bool {
    let mut computed_blocks = Vec::new();
    let mut curr_block_size = 0;
    for char in map.chars() {
        if char == '#' {
            curr_block_size += 1;
        } else {
            if curr_block_size > 0 {
                computed_blocks.push(curr_block_size);
            }
            curr_block_size = 0;
        }
    }
    if curr_block_size > 0 {
        computed_blocks.push(curr_block_size);
    }
    computed_blocks == *wanted_blocks
}

fn solve_line(map: String, wanted_blocks: &Vec<usize>) -> usize {
    if !map.contains('?') {
        return if check_line(map, wanted_blocks) { 1 } else { 0 };
    }
    solve_line(map.replacen('?', ".", 1), &wanted_blocks)
        + solve_line(map.replacen('?', "#", 1), &wanted_blocks)
}

pub fn spring1(file_path: &str) -> usize {
    let maps = parse_file(file_path);
    let mut sum = 0;
    for map in maps {
        sum += solve_line(map.0, &map.1);
    }
    sum
}

pub fn spring2(file_path: &str) -> usize {
    let maps = parse_file(file_path);
    let mut sum = 0;
    for map in maps {
        let mut wanted = Vec::new();
        for _ in 0..5 {
            for i in &map.1 {
                wanted.push(*i);
            }
        }
        sum += solve_line(vec![map.0; 5].join("?"), &wanted);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::{check_results, load_results};

    #[test]
    fn test_check_line() {
        assert_eq!(check_line(String::from("#.#.###"), &vec![1, 1, 3]), true);
        assert_eq!(
            check_line(String::from(".#.###.#.######"), &vec![1, 3, 1, 6]),
            true
        );
        assert_eq!(
            check_line(String::from(".#.###.#.######"), &vec![1, 3, 1, 8]),
            false
        );
    }

    #[test]
    fn p1() {
        check_results("d12", "p1", spring1);
    }

    #[test]
    fn p2() {
        check_results("d12", "p2", spring2);
    }
}
