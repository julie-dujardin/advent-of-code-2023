use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut map = Vec::new();
    for line in file.lines() {
        map.push(line.chars().collect());
    }
    map
}

fn spring1(file_path: &str) -> usize {
    0
}

fn spring2(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn p1() {
    //     assert_eq!(spring1("src/d12/input_test1.txt"), 21); // provided test
    //     assert_eq!(spring1("src/d12/input.txt"), 0);
    // }

    // #[test]
    // fn p2() {
    //     assert_eq!(spring2("src/d12/input_test1.txt"), 0); // provided test
    //     assert_eq!(spring2("src/d12/input.txt"), 0);
    // }
}
