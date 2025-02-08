use std::fs;
use std::str::Chars;

pub fn hash(chars: Chars) -> usize {
    let mut result = 0;

    for char in chars {
        result += char as usize;
        result *= 17;
        result %= 256;
    }

    result
}

fn parse_file(file_path: &str) -> Vec<String> {
    let file = fs::read_to_string(file_path).unwrap();
    let steps = file.split(',').map(|s| s.to_string()).collect();
    steps
}

pub fn library1(file_path: &str) -> usize {
    let mut result = 0;
    for step in parse_file(file_path) {
        result += hash(step.chars())
    }
    result
}

#[derive(Debug)]
struct Step {
    label: String,
    operation: char,
    focal_length: u8,
}

fn parse_step(step_str: String) -> Step {
    let mut step = Step {
        label: "".to_string(),
        operation: '-',
        focal_length: 0,
    };
    for char in step_str.chars() {
        if char == '-' || char == '=' {
            step.operation = char
        } else if char.is_numeric() {
            step.focal_length = char.to_string().parse::<u8>().unwrap()
        } else {
            step.label.push(char)
        }
    }
    step
}

pub fn library2(file_path: &str) -> usize {
    let mut boxes: Vec<Vec<Step>> = (0..256).map(|_| Vec::new()).collect();
    for step_str in parse_file(file_path) {
        let step = parse_step(step_str);
        let box_id = hash(step.label.chars());
        if step.operation == '=' {
            'insert: {
                for (i, stored_step) in boxes[box_id].iter().enumerate() {
                    if stored_step.label == step.label {
                        boxes[box_id][i] = step;
                        break 'insert;
                    }
                }
                boxes[box_id].push(step)
            }
        } else {
            let drop_id = 'pop: {
                for (i, stored_step) in boxes[box_id].iter().enumerate() {
                    if stored_step.label == step.label {
                        break 'pop Some(i);
                    }
                }
                None
            };
            if let Some(drop_) = drop_id {
                boxes[box_id].remove(drop_);
            }
        }
    }

    let mut result = 0;
    for (i, c_box) in boxes.iter().enumerate() {
        for (j, lens) in c_box.iter().enumerate() {
            result += (i + 1) * (j + 1) * lens.focal_length as usize
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        assert_eq!(hash("HASH".chars()), 52);

        check_results("d15", "p1", library1);
    }

    #[test]
    fn p2() {
        let step0 = parse_step("rn=1".to_string());
        assert_eq!(step0.label, "rn");
        assert_eq!(step0.operation, '=');
        assert_eq!(step0.focal_length, 1);

        let step1 = parse_step("cm-".to_string());
        assert_eq!(step1.label, "cm");
        assert_eq!(step1.operation, '-');
        assert_eq!(step1.focal_length, 0);

        check_results("d15", "p2", library2);
    }
}
