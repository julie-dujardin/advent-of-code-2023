use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut lines = Vec::new();
    for line in file.lines() {
        lines.push(line.chars().collect());
    }
    lines
}

// Direction: Up=0, Right=1, Down=2, Left=3

/// Make the specified ray move towards its specified direction, keeping its current direction
fn move_dir(i: isize, j: isize, direction: i32) -> (isize, isize, i32) {
    match direction {
        0 => (i, j - 1, direction),
        1 => (i + 1, j, direction),
        2 => (i, j + 1, direction),
        3 => (i - 1, j, direction),
        _ => panic!(),
    }
}

pub fn lava1(file_path: &str) -> usize {
    let map = parse_file(file_path);
    let mut rays = vec![(0isize, 0isize, 1)];

    let mut visited_positions = vec![vec![vec![false; 4]; map[0].len()]; map.len()];

    while !rays.is_empty() {
        let (x, y, direction) = rays.pop().unwrap();

        if x < 0 || y < 0 || (x as usize) >= map[0].len() || (y as usize) >= map.len() {
            continue;
        }

        if visited_positions[y as usize][x as usize][direction as usize] {
            // Already visited, skip
            continue;
        }
        visited_positions[y as usize][x as usize][direction as usize] = true;

        match map[y as usize][x as usize] {
            '.' => {
                // keep moving
                rays.push(move_dir(x, y, direction))
            }
            '\\' => {
                // ray hit the mirror -> redirect
                rays.push(move_dir(x, y, (direction - 3).abs()));
            }
            '/' => {
                // ray hit the mirror -> redirect
                rays.push(match direction {
                    0 => move_dir(x, y, 1),
                    1 => move_dir(x, y, 0),
                    2 => move_dir(x, y, 3),
                    3 => move_dir(x, y, 2),
                    _ => panic!(),
                })
            }
            '-' => {
                match direction {
                    0 | 2 => {
                        // ray hit the splitter -> spawn 2 new rays
                        rays.push((x + 1, y, 1));
                        rays.push((x - 1, y, 3));
                    }
                    1 | 3 => {
                        // no hit -> keep moving
                        rays.push(move_dir(x, y, direction))
                    }
                    _ => panic!(),
                }
            }
            '|' => {
                match direction {
                    1 | 3 => {
                        // ray hit the splitter -> spawn 2 new rays
                        rays.push((x, y + 1, 2));
                        rays.push((x, y - 1, 0));
                    }
                    0 | 2 => {
                        // no hit -> keep moving
                        rays.push(move_dir(x, y, direction))
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    visited_positions.iter().fold(0, |acc, line| {
        acc + line.iter().fold(0, |acc, directions| {
            if directions.iter().any(|visited| *visited) {
                acc + 1
            } else {
                acc
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d16", "p1", lava1);
    }

    // #[test]
    // fn p2() {
    //     check_results("d15", "p2", lava2);
    // }
}
