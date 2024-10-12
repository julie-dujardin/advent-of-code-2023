use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::{fs, thread};

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

fn solve_line1(map: String, wanted_blocks: &Vec<usize>) -> usize {
    if !map.contains('?') {
        return if check_line(map, wanted_blocks) { 1 } else { 0 };
    }
    solve_line1(map.replacen('?', ".", 1), &wanted_blocks)
        + solve_line1(map.replacen('?', "#", 1), &wanted_blocks)
}

pub fn spring1(file_path: &str) -> usize {
    let maps = parse_file(file_path);
    let (tx, rx) = mpsc::channel();
    for map in maps {
        let tx1: Sender<usize> = tx.clone();
        thread::spawn(move || {
            tx1.send(solve_line2(&map.0, &map.1, 0)).unwrap();
        });
    }
    drop(tx);
    rx.iter().sum()
}

pub fn spring2(file_path: &str) -> usize {
    let maps = parse_file(file_path);
    let (tx, rx) = mpsc::channel();
    for map in maps {
        let tx1 = tx.clone();
        thread::spawn(move || {
            let mut wanted = Vec::new();
            for _ in 0..5 {
                for i in &map.1 {
                    wanted.push(*i);
                }
            }
            tx1.send(solve_line2(&vec![map.0; 5].join("?"), &wanted, 0))
                .unwrap();
        });
    }
    drop(tx);
    rx.iter().sum()
}

fn solve_line2(map: &String, wanted_blocks: &[usize], search_start: usize) -> usize {
    if wanted_blocks.len() == 0 {
        if search_start < map.len() && map[search_start..map.len()].contains('#') {
            return 0;
        }
        return 1;
    }
    if search_start + wanted_blocks.iter().sum::<usize>() > map.len() {
        return 0;
    }
    let mut sum = 0;
    for i in search_start..map.len() {
        if i + wanted_blocks[0] > map.len() {
            return sum;
        }
        if map[search_start..i].contains('#') {
            return sum;
        }
        if (i == search_start || map.chars().nth(i - 1).unwrap() != '#')
            && !map[i..i + wanted_blocks[0]].contains('.')
            && (i + wanted_blocks[0] == map.len()
                || map.chars().nth(i + wanted_blocks[0]).unwrap() != '#')
        {
            sum += solve_line2(&map, &wanted_blocks[1..], i + wanted_blocks[0] + 1)
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::load_results;

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
        let expected = load_results("d12", "p1");
        assert_eq!(
            spring1("test-data/d12/input_test0.txt"),
            expected["input_test0"]
        );
        assert_eq!(
            spring1("test-data/d12/input_test1.txt"),
            expected["input_test1"]
        );
        assert_eq!(spring1("test-data/d12/input.txt"), expected["input"]);
    }

    #[test]
    fn p12() {
        let expected = load_results("d12", "p1");
        assert_eq!(
            spring1("test-data/d12/input_test0.txt"),
            expected["input_test0"]
        );
        assert_eq!(
            spring1("test-data/d12/input_test1.txt"),
            expected["input_test1"]
        );
        assert_eq!(spring1("test-data/d12/input.txt"), expected["input"]);
    }

    #[test]
    fn p2() {
        let expected = load_results("d12", "p2");
        assert_eq!(
            spring2("test-data/d12/input_test0.txt"),
            expected["input_test0"]
        );
        assert_eq!(
            spring2("test-data/d12/input_test1.txt"),
            expected["input_test1"]
        );
        // assert_eq!(spring2("test-data/d12/input.txt"), expected["input"]);
    }
}
