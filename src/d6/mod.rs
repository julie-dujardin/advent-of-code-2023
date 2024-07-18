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

fn race1(file_path: &str) -> i32 {
    let races = parse_file(file_path);

    let mut total = 1;

    for (time, record_distance) in races {
        let mut solution_start =
            (time as f64 - (((-time).pow(2) - 4 * record_distance) as f64).sqrt()) / 2.;
        let mut solution_end =
            (time as f64 + (((-time).pow(2) - 4 * record_distance) as f64).sqrt()) / 2.;

        // We need to do better, not just match the record
        if solution_start.fract() == 0. {
            solution_start += 1.;
        }

        total *= solution_end as i32 - solution_start as i32
    }

    println!("The total is {total}");
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(race1("src/d6/input_test1.txt"), 288); // provided test
        assert_eq!(race1("src/d6/input.txt"), 32076);
    }

    #[test]
    fn p2() {
        assert_eq!(race1("src/d6/input_test2.txt"), 71503); // provided test
        assert_eq!(race1("src/d6/input2.txt"), 34278221);
    }
}
