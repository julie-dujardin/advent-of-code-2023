use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn parse_file(file_path: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut maps = Vec::new();
    for line in file.lines() {
        let split: Vec<&str> = line.split(" ").collect();

        maps.push((
            split[0].chars().collect(),
            split[1].split(",").map(|c| c.parse().unwrap()).collect(),
        ));
    }
    maps
}

fn solve_line(
    map: &Vec<char>,
    wanted: &Vec<usize>,
    curr_group: usize,
    position: usize,
    curr_length: usize,
    memo: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(&cached) = memo.get(&(position, curr_group, curr_length)) {
        return cached;
    }

    if position == map.len() {
        if curr_group == wanted.len() && curr_length == 0 {
            return 1;
        } else if curr_group == wanted.len() - 1 && curr_length == *wanted.last().unwrap() {
            return 1;
        }
        return 0;
    }

    let mut result = 0;
    if let '?' | '#' = map[position] {
        result += solve_line(map, wanted, curr_group, position + 1, curr_length + 1, memo);
    };
    if let '?' | '.' = map[position] {
        if curr_length == 0 {
            result += solve_line(map, wanted, curr_group, position + 1, 0, memo);
        } else if curr_group < wanted.len() && curr_length == wanted[curr_group] {
            result += solve_line(map, wanted, curr_group + 1, position + 1, 0, memo)
        }
    };

    memo.insert((position, curr_group, curr_length), result);
    result
}

pub fn spring1(file_path: &str) -> usize {
    let maps = parse_file(file_path);
    let mut sum = 0;
    for (map, wanted) in maps {
        sum += solve_line(&map, &wanted, 0, 0, 0, &mut HashMap::new())
    }
    sum
}

pub fn spring2(file_path: &str) -> usize {
    let maps = parse_file(file_path);
    let mut sum = 0;
    for (map, wanted) in maps {
        let mut big_map = Vec::new();
        let mut big_wanted = Vec::new();
        for i in 0..5 {
            for i in map.iter() {
                big_map.push(*i);
            }
            if i < 4 {
                big_map.push('?');
            }
            for i in wanted.iter() {
                big_wanted.push(*i);
            }
        }
        sum += solve_line(&big_map, &big_wanted, 0, 0, 0, &mut HashMap::new());
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d12", "p1", spring1);
    }

    #[test]
    fn p2() {
        check_results("d12", "p2", spring2);
    }
}
