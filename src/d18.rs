use std::collections::HashMap;
use std::{fs, usize};

fn parse_file1(file_path: &str) -> Vec<(char, usize)> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut lines = Vec::new();
    for line in file.lines() {
        let elements = line.split(' ').collect::<Vec<&str>>();
        let value = (
            elements[0].chars().nth(0).unwrap(),
            elements[1].parse::<usize>().unwrap(),
        );
        lines.push(value);
    }
    lines
}

fn display_map(
    map: &HashMap<(usize, usize), char>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    border: usize,
) {
    for y in (min_y - border)..(max_y + 1 + border) {
        for x in (min_x - border)..(max_x + 1 + border) {
            let cell = map.get(&(x, y)).or(Some(&'.')).unwrap();
            print!("{cell}")
        }
        println!()
    }
}

pub fn lagoon1(file_path: &str) -> usize {
    let instructions = parse_file1(file_path);
    // init a map that's big enough, start in the middle
    let mut map = HashMap::new();
    let (mut curr_x, mut curr_y) = (2500, 2500);
    map.insert((curr_x, curr_y), 'S');

    let (mut min_x, mut max_x, mut min_y, mut max_y) = (2500, 2500, 2500, 2500);

    let mut prev_instruction = 'S';
    for (instruction, length) in instructions {
        for _ in 0..length {
            map.insert(
                (curr_x, curr_y),
                match (prev_instruction, instruction) {
                    ('S', _) => '#',
                    ('D' | 'U', 'D' | 'U') => '|',
                    ('R' | 'L', 'R' | 'L') => '-',
                    ('R', 'U') | ('D', 'L') => 'J',
                    ('R', 'D') | ('U', 'L') => '7',
                    ('D', 'R') | ('L', 'U') => 'L',
                    ('U', 'R') | ('L', 'D') => 'F',
                    (_, _) => panic!(),
                },
            );
            prev_instruction = instruction;
            (curr_x, curr_y) = match instruction {
                'U' => (curr_x, curr_y - 1),
                'R' => (curr_x + 1, curr_y),
                'D' => (curr_x, curr_y + 1),
                'L' => (curr_x - 1, curr_y),
                _ => panic!(),
            };
        }
        if curr_x < min_x {
            min_x = curr_x
        }
        if curr_y < min_y {
            min_y = curr_y
        }
        if curr_x > max_x {
            max_x = curr_x
        }
        if curr_y > max_y {
            max_y = curr_y
        }
    }

    // display_map(&map, min_x, max_x, min_y, max_y, 1);

    let mut capacity = 0;

    for y in min_y..(max_y + 1) {
        let mut edge_count = 0;
        for x in min_x..(max_x + 1) {
            if let Some(cell) = map.get(&(x, y)) {
                capacity += 1;

                if let 'J' | '|' | 'L' = cell {
                    edge_count += 1;
                }
            } else {
                if edge_count % 2 != 0 {
                    // map.insert((curr_x, curr_y), '!');
                    capacity += 1;
                }
            }
        }
    }

    // display_map(&map, min_x, max_x, min_y, max_y, 1);

    capacity
}

fn parse_file2(file_path: &str) -> Vec<(char, usize)> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut lines = Vec::new();
    for line in file.lines() {
        let elements = line.split(' ').collect::<Vec<&str>>();
        let value = (
            match elements[2].chars().nth(7).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!(),
            },
            usize::from_str_radix(&elements[2][2..7], 16).unwrap(),
        );
        lines.push(value);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d18", "p1", lagoon1);
    }

    // #[test]
    // fn p2() {
    //     check_results("d18", "p2", lagoon2);
    // }
}
