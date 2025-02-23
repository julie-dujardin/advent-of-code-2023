use std::collections::HashSet;
use std::fs;

fn parse_map(file_path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut lines = Vec::new();
    for line in file.lines() {
        lines.push(line.chars().collect());
    }
    lines
}

fn get_max_path_len(
    start_x: i32,
    start_y: i32,
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32)>,
    path_length: usize,
) -> usize {
    if start_x as usize == map[0].len() - 2 && start_y as usize == map.len() - 1 {
        return path_length;
    }

    visited.insert((start_x, start_y));
    let mut next_positions = Vec::new();

    for (check_x_offset, check_y_offset) in vec![(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let check_x = start_x + check_x_offset;
        let check_y = start_y + check_y_offset;
        // out-of-bounds
        if check_x < 0
            || check_y < 0
            || check_x >= map[0].len() as i32
            || check_y >= map.len() as i32
        {
            continue;
        }
        // can't be a wall or already visited
        let check_tile = map[check_y as usize][check_x as usize];
        if check_tile == '#' || visited.contains(&(check_x, check_y)) {
            continue;
        }
        // go the correct way
        if check_tile == '^' && check_y_offset != -1
            || check_tile == 'v' && check_y_offset != 1
            || check_tile == '>' && check_x_offset != 1
            || check_tile == '<' && check_x_offset != -1
        {
            continue;
        }

        next_positions.push((check_x, check_y))
    }

    if next_positions.len() == 0 {
        // fail state
        return 0;
    }

    let lengths = next_positions
        .iter()
        .map(|(x, y)| get_max_path_len(*x, *y, map, &mut visited.clone(), path_length + 1))
        .collect::<Vec<usize>>();
    println!("{:?}; {}; x={start_x}; y={start_y}", lengths, path_length);
    *lengths.iter().max().unwrap()
}

pub fn walk1(file_path: &str) -> usize {
    let map = parse_map(file_path);

    let mut visited = HashSet::new();
    get_max_path_len(1, 0, &map, &mut visited, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d23", "p1", walk1);
    }
}
