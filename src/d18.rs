use std::fs;

fn parse_file1(file_path: &str) -> Vec<(char, isize)> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut lines = Vec::new();
    for line in file.lines() {
        let elements = line.split(' ').collect::<Vec<&str>>();
        let value = (
            elements[0].chars().nth(0).unwrap(),
            elements[1].parse::<isize>().unwrap(),
        );
        lines.push(value);
    }
    lines
}

fn parse_file2(file_path: &str) -> Vec<(char, isize)> {
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
            isize::from_str_radix(&elements[2][2..7], 16).unwrap(),
        );
        lines.push(value);
    }
    lines
}

fn lagoon(instructions: Vec<(char, isize)>) -> usize {
    let (mut curr_x, mut curr_y): (isize, isize) = (0, 0);

    let mut area: isize = 0;
    let mut perimeter = 0;
    for (instruction, length) in instructions {
        let (new_x, new_y) = match instruction {
            'U' => (curr_x, curr_y - 1 * length),
            'R' => (curr_x + 1 * length, curr_y),
            'D' => (curr_x, curr_y + 1 * length),
            'L' => (curr_x - 1 * length, curr_y),
            _ => panic!(),
        };
        perimeter += length;

        area += curr_x * new_y - curr_y * new_x;

        (curr_x, curr_y) = (new_x, new_y);
    }

    area.abs() as usize / 2 + perimeter as usize / 2 + 1
}

pub fn lagoon1(file_path: &str) -> usize {
    let instructions = parse_file1(file_path);
    lagoon(instructions)
}

pub fn lagoon2(file_path: &str) -> usize {
    let instructions = parse_file2(file_path);
    lagoon(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d18", "p1", lagoon1);
    }

    #[test]
    fn p2() {
        check_results("d18", "p2", lagoon2);
    }
}
