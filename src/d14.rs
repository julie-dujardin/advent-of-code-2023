use std::collections::HashMap;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};

fn parse_file(file_path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        for (i, char) in line.chars().enumerate() {
            if map.len() <= i {
                map.push(Vec::new());
            }
            (&mut map[i]).push(char)
        }
    }
    map
}

pub fn dishes1(file_path: &str) -> usize {
    let map = parse_file(file_path);

    let mut load = 0;

    for mut column in map {
        let mut back_ptr = 0;
        let mut fwd_ptr = 1;
        while back_ptr < column.len() {
            if column[back_ptr] == 'O' {
                load += column.len() - back_ptr;
                fwd_ptr += 1;
            } else if column[back_ptr] == '#' {
                fwd_ptr += 1;
            } else {
                while fwd_ptr < column.len() {
                    if column[fwd_ptr] == 'O' {
                        load += column.len() - back_ptr;
                        column[fwd_ptr] = '.';
                        column[back_ptr] = 'O';
                        break;
                    }
                    if column[fwd_ptr] == '#' {
                        break;
                    }
                    fwd_ptr += 1;
                }
            }

            back_ptr += 1;
        }
    }

    load
}

fn parse_file2(file_path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut lines = Vec::new();
    for line in file.lines() {
        lines.push(
            line.chars()
                .collect(),
        );
    }
    lines
}

fn get_load(map: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                load += map.len() - y;
            }
        }
    }
    load
}

enum Direction {
    North,
    West,
    South,
    East,
}

fn get_ptr_max(map: &Vec<Vec<char>>, direction: &Direction) -> usize {
    match direction {
        Direction::North | Direction::South => {
            map.len()
        }
        Direction::West | Direction::East => {
            map[0].len()
        }
    }
}

fn get_map_at(map: &Vec<Vec<char>>, direction: &Direction, x: usize, y: usize) -> char {
    match direction {
        Direction::North => {
            map[y][x]
        }
        Direction::West => {
            map[x][y]
        }
        Direction::South => {
            map[map.len() - y - 1][x]
        }
        Direction::East => {
            map[x][map[0].len() - y - 1]
        }
    }
}

fn set_map_at(map: &mut Vec<Vec<char>>, direction: &Direction, x: usize, y: usize, value: char) {
    let map_col_len = map.len();
    let map_line_len = map[0].len();
    match direction {
        Direction::North => {
            map[y][x] = value;
        }
        Direction::West => {
            map[x][y] = value;
        }
        Direction::South => {
            map[map_col_len - y - 1][x] = value;
        }
        Direction::East => {
            map[x][map_line_len - y - 1] = value;
        }
    }
}

fn rotate(map: &mut Vec<Vec<char>>, direction: Direction) {
    let ptr_max = get_ptr_max(&map, &direction);
    for x in 0..map[0].len() {
        let mut y_back_ptr = 0;
        let mut y_fwd_ptr = 1;
        while y_back_ptr < ptr_max && y_fwd_ptr < ptr_max {
            match get_map_at(&map, &direction, x, y_fwd_ptr) {
                'O' => {
                    while get_map_at(&map, &direction, x, y_back_ptr) != '.' && y_back_ptr + 1 < y_fwd_ptr {
                        y_back_ptr += 1
                    }
                    if get_map_at(&map, &direction, x, y_back_ptr) == '.' {
                        set_map_at(map, &direction, x, y_back_ptr, 'O');
                        set_map_at(map, &direction, x, y_fwd_ptr, '.');
                        y_back_ptr += 1;
                    }
                    y_fwd_ptr += 1;
                },
                '#' => {
                    y_back_ptr = y_fwd_ptr;
                    y_fwd_ptr += 1;
                },
                '.' => {
                    y_fwd_ptr += 1;
                }
                _ => unreachable!()
            }
        }
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map.iter() {
        for cell in line.iter() {
            print!("{}", cell)
        }
        println!()
    }
    println!()
}

pub fn dishes10(file_path: &str) -> usize {
    let mut map = parse_file2(file_path);
    rotate(&mut map, Direction::North);
    get_load(&map)
}

pub fn dishes11(file_path: &str) -> usize {
    let mut map = parse_file2(file_path);
    print_map(&map);
    for dir in vec![Direction::North, Direction::West, Direction::South, Direction::East] {
        rotate(&mut map, dir);
        print_map(&map);
    }
    get_load(&map)
}


pub fn dishes2(file_path: &str) -> usize {
    let mut map = parse_file2(file_path);
    let mut states = HashMap::new();
    let mut weights = HashMap::new();

    for i in 0..1_000_000_000 {
        for dir in vec![Direction::North, Direction::West, Direction::South, Direction::East] {
            rotate(&mut map, dir);
        }

        let mut hash = DefaultHasher::new();
        map.hash(&mut hash);
        let curr_hash = hash.finish();
        if let Some(iter_start_idx) = states.get(&curr_hash){
            let loop_len = i - iter_start_idx;
            let result_idx = (1_000_000_000 - iter_start_idx) % loop_len + iter_start_idx - 1;
            return weights[&result_idx];
        }
        else {
            states.insert(curr_hash, i);
            weights.insert(i, get_load(&map));
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d14", "p1", dishes1);
    }

    #[test]
    fn p10() {
        check_results("d14", "p1", dishes10);
    }

    // #[test]
    // fn p11() {
    //     dishes11("test-data/d14/input_test1.txt");
    // }

    #[test]
    fn p2() {
        check_results("d14", "p2", dishes2);
    }
}
