use std::collections::HashMap;
use std::fs;

fn coord_generator(start_x: i32, end_x: i32, line_y: i32) -> Vec<(i32, i32)> {
    let mut coords: Vec<(i32, i32)> = vec![(start_x - 1, line_y), (end_x, line_y)];

    for x in start_x - 1..end_x + 1 {
        coords.push((x, line_y - 1));
        coords.push((x, line_y + 1));
    }

    coords
}

fn gears1(file_path: &str) -> i32 {
    let mut sum = 0;
    let file = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = file.lines().collect();
    for (y, line) in lines.iter().enumerate() {
        let mut number_start_index = -1;
        for (x, char) in line.chars().enumerate() {
            // Keep track of if the current char is a digit, and if so the first visited digit in the current number
            if char.is_numeric() && number_start_index < 0 {
                number_start_index = x as i32;
            }
            // If we're just about to finish our number, due to line length limit, or the next char not beihg a digit
            if number_start_index >= 0
                && (x + 1 == line.len() || !line.chars().nth(x + 1).unwrap().is_numeric())
            {
                let number: i32 = line[number_start_index as usize..x + 1].parse().unwrap();

                // Iterate over every coordinate around this number
                for (check_x, check_y) in
                    coord_generator(number_start_index, x as i32 + 1, y as i32)
                {
                    // If they're in range,
                    if check_x >= 0
                        && check_x < line.len() as i32
                        && check_y >= 0
                        && check_y < lines.len() as i32
                    {
                        let char_check: char = lines[check_y as usize]
                            .chars()
                            .nth(check_x as usize)
                            .unwrap();
                        // and not a '.', then add them to sum
                        if char_check != '.' {
                            sum += number;
                            break;
                        }
                    }
                }

                number_start_index = -1;
            }
        }
    }
    println!("The sum of part numbers is {sum}");
    sum
}

fn gears2(file_path: &str) -> i32 {
    let mut gear_candidates: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    let file = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = file.lines().collect();
    for (y, line) in lines.iter().enumerate() {
        let mut number_start_index = -1;
        for (x, char) in line.chars().enumerate() {
            // Keep track of if the current char is a digit, and if so the first visited digit in the current number
            if char.is_numeric() && number_start_index < 0 {
                number_start_index = x as i32;
            }
            // If we're just about to finish our number, due to line length limit, or the next char not beihg a digit
            if number_start_index >= 0
                && (x + 1 == line.len() || !line.chars().nth(x + 1).unwrap().is_numeric())
            {
                let number: i32 = line[number_start_index as usize..x + 1].parse().unwrap();

                // Iterate over every coordinate around this number
                for (check_x, check_y) in
                    coord_generator(number_start_index, x as i32 + 1, y as i32)
                {
                    // If they're in range,
                    if check_x >= 0
                        && check_x < line.len() as i32
                        && check_y >= 0
                        && check_y < lines.len() as i32
                    {
                        let char_check: char = lines[check_y as usize]
                            .chars()
                            .nth(check_x as usize)
                            .unwrap();
                        // and a '*', then add them to the number list for the current gear candidate
                        if char_check == '*' {
                            gear_candidates
                                .entry((check_y, check_x))
                                .and_modify(|e| e.push(number))
                                .or_insert(vec![number]);
                            break;
                        }
                    }
                }

                number_start_index = -1;
            }
        }
    }

    // Compute the sum of all gear candidates with exactly 2 numbers
    let mut sum = 0;
    for nums in gear_candidates.values() {
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    }

    println!("The sum of gear ratios is {sum}");
    sum
}

mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(gears1("src/d3/input_test0.txt"), 4533); // select all
        assert_eq!(gears1("src/d3/input_test1.txt"), 4361); // provided test
        assert_eq!(gears1("src/d3/input_test2.txt"), 3859); // numbers at end of line
        assert_eq!(gears1("src/d3/input.txt"), 537832);
    }
    #[test]
    fn p2() {
        assert_eq!(gears2("src/d3/input_test1.txt"), 467835); // provided test
        assert_eq!(gears2("src/d3/input_test2.txt"), 451490); // numbers at end of line
        assert_eq!(gears2("src/d3/input.txt"), 81939900);
    }
}
