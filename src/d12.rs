use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut map = Vec::new();
    for line in file.lines() {
        map.push(line.chars().collect());
    }
    map
}

pub fn spring1(file_path: &str) -> usize {
    0
}

pub fn spring2(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn p1() {
    // let (expected_p1, _) = load_results("d12").unwrap();
    // assert_eq!(spring1("test-data/d12/input_test1.txt"), expected_p1["input_test1"]);
    // assert_eq!(spring1("test-data/d12/input.txt"), expected_p1["input"]);
    // }

    // #[test]
    // fn p2() {
    // let (_, expected_p2) = load_results("d12").unwrap();
    // assert_eq!(spring2("test-data/d12/input_test1.txt"), expected_p2["input_test1"]);
    // assert_eq!(spring2("test-data/d12/input.txt"), expected_p2["input"]);
    // }
}
