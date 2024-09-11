use std::fs;

fn numbers_str_to_vec(s: &str) -> Vec<isize> {
    let mut nums = Vec::new();
    for number_s in s.split(' ').filter(|&x| !x.is_empty()) {
        nums.push(number_s.parse().unwrap());
    }
    nums
}

fn parse_file(file_path: &str) -> Vec<(isize, isize)> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut lines = file.lines();
    let line1 = numbers_str_to_vec(&lines.next().unwrap()[11..]);
    let line2 = numbers_str_to_vec(&lines.next().unwrap()[11..]);
    let mut races = Vec::new();
    for (i, num1) in line1.iter().enumerate() {
        races.push((*num1, line2[i]));
    }
    races
}

pub fn race(file_path: &str) -> usize {
    let races = parse_file(file_path);

    let mut total = 1;

    for (time, record_distance) in races {
        let mut solution_start =
            (time as f64 - (((-time).pow(2) - 4 * record_distance) as f64).sqrt()) / 2.;
        let solution_end =
            (time as f64 + (((-time).pow(2) - 4 * record_distance) as f64).sqrt()) / 2.;

        // We need to do better, not just match the record
        if solution_start.fract() == 0. {
            solution_start += 1.;
        }

        total *= solution_end as i32 - solution_start as i32
    }

    println!("The total is {total}");
    total as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::load_results;

    #[test]
    fn p1() {
        let expected_p1 = load_results("d6", "p1");
        assert_eq!(
            race("test-data/d6/input_test1.txt"),
            expected_p1["input_test1"]
        );
        assert_eq!(race("test-data/d6/input.txt"), expected_p1["input"]);
    }

    #[test]
    fn p2() {
        let expected_p2 = load_results("d6", "p2");
        assert_eq!(
            race("test-data/d6/input_test2.txt"),
            expected_p2["input_test2"]
        );
        assert_eq!(race("test-data/d6/input2.txt"), expected_p2["input2"]);
    }
}
