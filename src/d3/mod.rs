use std::fs;

pub fn test_gears() {
    assert_eq!(gears1("src/d3/input_test0.txt"), 4533);  // select all
    assert_eq!(gears1("src/d3/input_test1.txt"), 4361);  // provided test
    assert_eq!(gears1("src/d3/input_test2.txt"), 3859);  // numbers at end of line
    assert_eq!(gears1("src/d3/input.txt"), 537832);
}

fn coord_generator(start_x: i32, end_x: i32, line_y: i32) -> Vec<(i32, i32)> {
    let mut coords: Vec<(i32, i32)> = vec![
        (start_x -1, line_y),
        (end_x, line_y),
    ];

    for x in (start_x -1 .. end_x + 1) {
        coords.push((x, line_y - 1));
        coords.push((x, line_y + 1));
    }

    coords
}

fn gears1(input_file: &str) -> i32 {
    let mut sum = 0;
    let binding = fs::read_to_string(input_file).unwrap();
    let file: Vec<&str> = binding.lines().collect();
    for (y, line) in file.iter().enumerate() {
        let mut number_start_index = -1;
        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() {
                if number_start_index < 0 {
                    number_start_index = x as i32;
                }
            }
            if number_start_index >= 0 && (x + 1 == line.len() || !line.chars().nth(x+1).unwrap().is_numeric()) {
                if number_start_index >= 0 {
                    let number: i32 = line[number_start_index as usize..x+1].parse().unwrap();

                    let mut excluded: bool = true;

                    for (check_x, check_y) in coord_generator(number_start_index, x as i32+1, y as i32) {
                        if check_x >= 0 && check_x < line.len() as i32 && check_y >= 0 && check_y < file.len() as i32 {
                            let char_check: char = file[check_y as usize].chars().nth(check_x as usize).unwrap();
                            if char_check != '.' {
                                sum += number;
                                excluded = false;
                                break
                            }
                        }
                    }

                    number_start_index = -1;
                }
            }
        }
    }
    println!("The sum of part numbers is {sum}");
    sum
}