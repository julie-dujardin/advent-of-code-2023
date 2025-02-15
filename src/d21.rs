use std::collections::HashSet;
use std::fs;

fn parse_file(file_path: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let file = fs::read_to_string(file_path).unwrap();
    let mut map = Vec::new();
    let mut coords = None;
    for (y, line) in file.lines().enumerate() {
        let mut line_chars = Vec::new();
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                coords = Some((x, y));
                line_chars.push('.');
            } else {
                line_chars.push(char);
            }
        }
        map.push(line_chars);
    }
    (map, coords.unwrap())
}

pub fn steps1(file_path: &str, num_steps: usize) -> usize {
    let (map, start_coords) = parse_file(file_path);

    let mut possible_positions = HashSet::new();
    possible_positions.insert(start_coords);

    for _ in 0..num_steps {
        let mut next_positions = HashSet::new();
        for (curr_x, curr_y) in possible_positions.drain() {
            for (check_x_offset, check_y_offset) in vec![(0, -1), (1, 0), (0, 1), (-1, 0)] {
                let check_x = curr_x as i32 + check_x_offset;
                let check_y = curr_y as i32 + check_y_offset;

                if check_x < 0
                    || check_y < 0
                    || check_x >= map[0].len() as i32
                    || check_y >= map.len() as i32
                {
                    continue;
                }

                if map[check_y as usize][check_x as usize] != '.' {
                    continue;
                }

                next_positions.insert((check_x as usize, check_y as usize));
            }
        }
        possible_positions = next_positions;
    }

    possible_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::load_results;

    #[test]
    fn p1() {
        let expected_p1 = load_results("d21", "p1");
        assert_eq!(
            steps1("test-data/d21/input_test1.txt", 6),
            expected_p1["input_test1"]
        );
        assert_eq!(steps1("test-data/d21/input.txt", 64), expected_p1["input"]);
    }
}
