use std::fs;

fn parse_file(file_path: &str) -> Vec<(String, Vec<usize>)> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut map = Vec::new();
    for line in file.lines() {
        let split: Vec<&str> = line.split(" ").collect();

        map.push((
            split[0].to_string(),
            split[1].split(",").map(|c| c.parse().unwrap()).collect(),
        ));
    }
    map
}

pub fn spring1(file_path: &str) -> usize {
    let map = parse_file(file_path);
    0
}

pub fn spring2(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::load_results;

    // #[test]
    // fn p1() {
    //     let expected_p1 = load_results("d12", "p1");
    //     assert_eq!(
    //         spring1("test-data/d12/input_test1.txt"),
    //         expected_p1["input_test1"]
    //     );
    //     assert_eq!(spring1("test-data/d12/input.txt"), expected_p1["input"]);
    // }

    // #[test]
    // fn p2() {
    // let expected_p2 = load_results("d12", "p2");
    // assert_eq!(spring2("test-data/d12/input_test1.txt"), expected_p2["input_test1"]);
    // assert_eq!(spring2("test-data/d12/input.txt"), expected_p2["input"]);
    // }
}
