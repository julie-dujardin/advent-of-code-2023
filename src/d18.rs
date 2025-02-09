use std::fs;

fn parse_file(file_path: &str) -> Vec<(char, usize, String)> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut lines = Vec::new();
    for line in file.lines() {
        let elements = line.split(' ').collect::<Vec<&str>>();
        let value = (
            elements[0].chars().nth(0).unwrap(),
            elements[1].parse::<usize>().unwrap(),
            elements[2].to_string(),
        );
        lines.push(value);
    }
    lines
}

fn display_map(
    map: &Vec<Vec<char>>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    border: usize,
) {
    for y in (min_y - border)..(max_y + 1 + border) {
        for x in (min_x - border)..(max_x + 1 + border) {
            let cell = map[y][x];
            print!("{cell}")
        }
        println!()
    }
}

pub fn lagoon1(file_path: &str) -> usize {
    let instructions = parse_file(file_path);
    // init a map that's big enough, start in the middle
    let mut map = vec![vec!['.'; 5000]; 5000];
    let (mut curr_x, mut curr_y) = (2500, 2500);
    map[curr_y][curr_x] = 'S';

    let (mut min_x, mut max_x, mut min_y, mut max_y) = (2500, 2500, 2500, 2500);

    let mut prev_instruction = 'S';
    for (instruction, length, _) in instructions {
        for _ in 0..length {
            map[curr_y][curr_x] = match (prev_instruction, instruction) {
                ('S', _) => '#',
                ('D' | 'U', 'D' | 'U') => '|',
                ('R' | 'L', 'R' | 'L') => '-',
                ('R', 'U') | ('D', 'L') => 'J',
                ('R', 'D') | ('U', 'L') => '7',
                ('D', 'R') | ('L', 'U') => 'L',
                ('U', 'R') | ('L', 'D') => 'F',
                (_, _) => panic!(),
            };
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
            if map[y][x] != '.' {
                capacity += 1;

                if let 'J' | '|' | 'L' = map[y][x] {
                    edge_count += 1;
                }
            } else {
                if edge_count % 2 != 0 {
                    map[y][x] = '!';
                    capacity += 1;
                }
            }
        }
    }

    // display_map(&map, min_x, max_x, min_y, max_y, 1);

    capacity
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
