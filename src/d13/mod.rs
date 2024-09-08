use std::cmp::min;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn parse_file(file_path: &str) -> Vec<(Vec<u64>, Vec<u64>)> {
    // Compute the hashes of the lines & columns of each pattern
    // (hashes of lines, hashes of columns)

    let file = fs::read_to_string(file_path).unwrap();
    let mut hashes = Vec::new();
    for pattern in file.split("\n\n") {
        let mut pattern_hashes = (Vec::new(), Vec::new());

        let pattern_lines = pattern.lines().collect::<Vec<_>>();
        for line in &pattern_lines {
            pattern_hashes.0.push(calculate_hash(&line))
        }

        for x in 0..pattern_lines[0].len() {
            let mut column = String::new();
            for y in 0..pattern_lines.len() {
                column.push(pattern_lines[y].chars().nth(x).unwrap())
            }
            pattern_hashes.1.push(calculate_hash(&column))
        }

        hashes.push(pattern_hashes);
    }
    hashes
}

fn check_hashes(hashes: Vec<u64>) -> usize {
    let mut prev_hash = *hashes.first().unwrap();

    for i in 1..hashes.len() {
        let curr_hash = hashes[i];

        if curr_hash == prev_hash {
            let mut found_mirror = true;
            for check_delta in 1..min(hashes.len() - i, i) {
                if hashes[i + check_delta] != hashes[i - 1 - check_delta] {
                    found_mirror = false;
                    break;
                }
            }
            if found_mirror {
                return i;
            }
        }

        prev_hash = curr_hash;
    }
    0
}

fn mirrors1(file_path: &str) -> usize {
    let hashes = parse_file(file_path);
    let mut sum = 0;

    for pattern_hashes in hashes {
        sum += check_hashes(pattern_hashes.1) + 100 * check_hashes(pattern_hashes.0);
    }

    sum
}

fn mirrors2(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(mirrors1("src/d13/input_test1.txt"), 405); // provided test
        assert_eq!(mirrors1("src/d13/input.txt"), 33975);
    }

    // #[test]
    // fn p2() {
    //     assert_eq!(mirrors2("src/d13/input_test1.txt"), 0); // provided test
    //     assert_eq!(mirrors2("src/d13/input.txt"), 0);
    // }
}
