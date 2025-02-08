use std::fs;
use std::str::Chars;

pub fn hash(chars: Chars) -> usize {
    let mut result = 0;

    for char in chars{
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
    for step in parse_file(file_path){
        result += hash(step.chars())
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::load_results;

    #[test]
    fn p1() {
        assert_eq!(hash("HASH".chars()), 52);

        let expected = load_results("d15", "p1");
        assert_eq!(
            library1("test-data/d15/input_test1.txt"),
            expected["input_test1"]
        );
        assert_eq!(library1("test-data/d15/input.txt"), expected["input"]);
    }

    // #[test]
    // fn p2() {
    //     let expected = load_results("d15", "p2");
    //     assert_eq!(
    //         hash2("test-data/d15/input_test1.txt"),
    //         expected["input_test1"]
    //     );
    //     assert_eq!(hash2("test-data/d15/input.txt"), expected["input"]);
    // }
}