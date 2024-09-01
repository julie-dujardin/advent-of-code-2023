use std::fs;
use std::str::Chars;

fn parse_file(file_path: &str) -> Vec<Chars> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut lines = Vec::new();
    for line in file.lines() {
        lines.push(line.chars());
    }
    lines
}

fn maze1(file_path: &str) -> isize {
    0
}

fn maze2(file_path: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(maze1("src/d10/input_test1_simple.txt"), 4); // provided test
        assert_eq!(maze1("src/d10/input_test1.txt"), 4); // provided test
        assert_eq!(maze1("src/d10/input_test2_simple.txt"), 8); // provided test
        assert_eq!(maze1("src/d10/input_test2.txt"), 8); // provided test
        assert_eq!(maze1("src/d10/input.txt"), 1);
    }

    #[test]
    fn p2() {
        assert_eq!(maze2("src/d10/input_test1_simple.txt"), 2); // provided test
        assert_eq!(maze2("src/d10/input.txt"), 1);
    }
}
