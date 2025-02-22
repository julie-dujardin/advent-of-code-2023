use std::collections::HashSet;
use std::fs;

fn parse_file(file_path: &str) -> (Vec<Vec<char>>, (i32, i32)) {
    let file = fs::read_to_string(file_path).unwrap();
    let mut map = Vec::new();
    let mut coords = None;
    for (y, line) in file.lines().enumerate() {
        let mut line_chars = Vec::new();
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                coords = Some((x as i32, y as i32));
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
                let check_x = curr_x + check_x_offset;
                let check_y = curr_y + check_y_offset;

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

                next_positions.insert((check_x, check_y));
            }
        }
        possible_positions = next_positions;
    }

    possible_positions.len()
}

pub fn steps2_bruteforce(file_path: &str, num_steps: usize) -> usize {
    let (map, start_coords) = parse_file(file_path);

    let mut possible_positions = HashSet::new();
    possible_positions.insert(start_coords);

    for i in 0..num_steps {
        println!("{i} {}", possible_positions.len());
        let mut next_positions = HashSet::new();
        for (curr_x, curr_y) in possible_positions.drain() {
            for (check_x_offset, check_y_offset) in vec![(0, -1), (1, 0), (0, 1), (-1, 0)] {
                let mut check_x = (curr_x + check_x_offset) % map[0].len() as i32;
                let mut check_y = (curr_y + check_y_offset) % map.len() as i32;

                if check_x < 0 {
                    check_x += map[0].len() as i32
                }
                if check_y < 0 {
                    check_y += map.len() as i32
                }

                if map[check_y as usize][check_x as usize] != '.' {
                    continue;
                }

                next_positions.insert((curr_x + check_x_offset, curr_y + check_y_offset));
            }
        }
        possible_positions = next_positions;
    }

    possible_positions.len()
}

pub fn steps2(num_steps: usize) -> f64 {
    // https://www.dcode.fr/lagrange-interpolating-polynomial
    15549. / 17161. * num_steps.pow(2) as f64
        + 26684. / 17161. * num_steps as f64
        + 236838. / 17161.
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

    #[test]
    fn p2_0() {
        let expected_p2 = load_results("d21", "p2");
        assert_eq!(
            steps2_bruteforce("test-data/d21/input_test1.txt", 6),
            expected_p2["input_test1_6"]
        );
        assert_eq!(
            steps2_bruteforce("test-data/d21/input_test1.txt", 10),
            expected_p2["input_test1_10"]
        );
        assert_eq!(
            steps2_bruteforce("test-data/d21/input_test1.txt", 50),
            expected_p2["input_test1_50"]
        );
        assert_eq!(
            steps2_bruteforce("test-data/d21/input_test1.txt", 100),
            expected_p2["input_test1_100"]
        );
        // assert_eq!(steps2_bruteforce("test-data/d21/input_test1.txt", 500), expected_p2["input_test1_500"]);
        // assert_eq!(steps2_bruteforce("test-data/d21/input_test1.txt", 1000), expected_p2["input_test1_1000"]);
        // assert_eq!(steps2_bruteforce("test-data/d21/input_test1.txt", 5000), expected_p2["input_test1_5000"]);
    }

    // #[test]
    // fn p2_1() {
    //     steps2_bruteforce("test-data/d21/input.txt", 500);
    // }

    #[test]
    fn p2_2() {
        let expected_p2 = load_results("d21", "p2");
        assert_eq!(steps2(26501365) as usize, expected_p2["input"]);
    }
}
