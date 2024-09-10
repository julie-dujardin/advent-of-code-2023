use std::cmp::min;
use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<Vec<char>>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut maps = Vec::new();
    for pattern in file.split("\n\n") {
        let mut map = Vec::new();
        for line in pattern.lines() {
            map.push(line.chars().collect());
        }
        maps.push(map)
    }
    maps
}

fn get_pos(map: &Vec<Vec<char>>, horizontal: bool, i: usize, j: usize) -> char {
    if horizontal {
        map[i][j]
    } else {
        map[j][i]
    }
}

fn get_diff_count_for_lines(
    map: &Vec<Vec<char>>,
    horizontal: bool,
    i1: usize,
    i2: usize,
    wanted_diffs: usize,
) -> usize {
    let l2_max = if horizontal {
        map.first().unwrap().len()
    } else {
        map.len()
    };
    let mut diff_count = 0;
    for j in 0..l2_max {
        if get_pos(map, horizontal, i1, j) != get_pos(map, horizontal, i2, j) {
            diff_count += 1;
            if diff_count > wanted_diffs {
                return diff_count;
            }
        }
    }
    diff_count
}

fn get_line_count_before_mirror(
    map: &Vec<Vec<char>>,
    horizontal: bool,
    wanted_diffs: usize,
) -> usize {
    let l1_max = if horizontal {
        map.len()
    } else {
        map.first().unwrap().len()
    };

    for i in 1..l1_max {
        let mut diff_count = get_diff_count_for_lines(map, horizontal, i, i - 1, wanted_diffs);
        if diff_count <= wanted_diffs {
            for check_delta in 1..min(l1_max - i, i) {
                diff_count += get_diff_count_for_lines(
                    map,
                    horizontal,
                    i + check_delta,
                    i - 1 - check_delta,
                    wanted_diffs,
                );
                if diff_count > wanted_diffs {
                    break;
                }
            }
        }
        if diff_count == wanted_diffs {
            return i;
        }
    }

    0
}

fn mirrors(file_path: &str, acceptable_diffs: usize) -> usize {
    let maps = parse_file(file_path);
    let mut sum = 0;

    for map in maps {
        sum += 100 * get_line_count_before_mirror(&map, true, acceptable_diffs)
            + get_line_count_before_mirror(&map, false, acceptable_diffs);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(mirrors("test-data/d13/input_test1.txt", 0), 405); // provided test
        assert_eq!(mirrors("test-data/d13/input.txt", 0), 33975);
    }

    #[test]
    fn p2() {
        assert_eq!(mirrors("test-data/d13/input_test1.txt", 1), 400); // provided test
        assert_eq!(mirrors("test-data/d13/input.txt", 1), 29083);
    }
}
