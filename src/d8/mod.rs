use std::collections::HashMap;
use std::fs;

// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(Vec::from(&nums[1..]));
    a * b / gcd_of_two_numbers(a, b)
}

// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn parse_file(file_path: &str) -> (String, HashMap<String, (String, String)>) {
    let file = fs::read_to_string(file_path).unwrap();
    let mut file_lines = file.lines();
    let instructions = String::from(file_lines.next().unwrap());
    file_lines.next();
    let mut nodes = HashMap::new();
    for line in  file_lines{
        nodes.insert(String::from(&line[0..3]), (String::from(&line[7..10]),String::from(&line[12..15])));
    }
    (instructions, nodes)
}

fn maze1(file_path: &str) -> usize {
    let (instructions, nodes) = parse_file(file_path);

    let mut curr_node: &String = &String::from("AAA");
    let mut steps = 0;
    while curr_node != &String::from("ZZZ") {
        let curr_instruction = instructions.chars().nth(steps % instructions.len()).unwrap();

        curr_node = match curr_instruction {
            'L' => &nodes.get(curr_node).unwrap().0,
            'R' => &nodes.get(curr_node).unwrap().1,
            _ => panic!("Instruction is not either L or R")
        };

        steps += 1;
    }

    steps
}

fn solve_path2(instructions: &String, nodes: &HashMap<String, (String, String)>, start_node: &String) -> usize {
    let mut curr_node = start_node;
    let mut steps = 0;
    while curr_node.chars().nth(2).unwrap() != 'Z' {
        let curr_instruction = instructions.chars().nth(steps % instructions.len()).unwrap();

        curr_node = match curr_instruction {
            'L' => &nodes.get(curr_node).unwrap().0,
            'R' => &nodes.get(curr_node).unwrap().1,
            _ => panic!("Instruction is not either L or R")
        };

        steps += 1;
    }

    steps
}

fn maze2(file_path: &str) -> usize {
    let (instructions, nodes) = parse_file(file_path);

    let mut steps: Vec<usize> = Vec::new();
    for node in nodes.keys().filter(|&x| x.chars().nth(2).unwrap() == 'A') {
        steps.push(solve_path2(&instructions, &nodes, node));
    };

    lcm(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(maze1("src/d8/input_test1.txt"), 2); // provided test
        assert_eq!(maze1("src/d8/input_test2.txt"), 6); // provided test
        assert_eq!(maze1("src/d8/input.txt"), 20777);
    }

    #[test]
    fn p2() {
        assert_eq!(maze2("src/d8/input_test3.txt"), 6); // provided test
        assert_eq!(maze2("src/d8/input.txt"), 13289612809129);
    }
}
