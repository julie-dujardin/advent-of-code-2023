use std::collections::HashMap;
use std::fs;

fn trebuchet1(file_path: &str) -> i32 {
    let mut sum= 0;
    // TODO line buffering
    // TODO line multithreading
    for line in fs::read_to_string(file_path).unwrap().lines(){
        let mut number = String::new();
        let digits: Vec<&str> = line.matches(char::is_numeric).collect();
        number.push_str(digits[0]);
        number.push_str(digits.last().unwrap());
        sum += number.parse::<i32>().unwrap()
    }
    println!("The calibration value is {sum}");
    sum
}

fn find_text_digit(line: &str, rev: bool) -> (usize, char) {
    let digits = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut li = if rev {0} else {line.len() + 1};
    let mut ln = '0';
    for (digit, int) in &digits {
        let index = if rev {line.rfind(digit)} else {line.find(digit)};
        if index.is_some() && ((!rev && index.unwrap() < li) || (rev && index.unwrap() >= li)) {
            li = index.unwrap();
            ln = *int;
        }
    }
    (li, ln)
}

fn trebuchet2(file_path: &str) -> i32 {
    let mut sum= 0;
    for line in fs::read_to_string(file_path).unwrap().lines(){
        let mut number = String::new();

        let (li, ln) = find_text_digit(line, false);
        for (i, a) in line.chars().enumerate() {
            if i >= li {
                break
            }
            if a.is_ascii_digit() {
                number.push(a);
                break;
            }
        }
        if number.is_empty() {
            number.push(ln)
        }

        let (ri, rn) = find_text_digit(line, true);
        for (i, a) in line.chars().rev().enumerate() {
            if (line.len() - i) <= ri {
                break
            }
            if a.is_ascii_digit() {
                number.push(a);
                break;
            }
        }
        if number.len() == 1 {
            number.push(rn)
        }

        sum += number.parse::<i32>().unwrap()
    }
    println!("The corrected calibration value is {sum}");
    sum
}

mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(trebuchet1("src/d1/input_test1.txt"), 142);
        assert_eq!(trebuchet1("src/d1/input.txt"), 55123);
    }
    #[test]
    fn p2() {
        assert_eq!(trebuchet2("src/d1/input_test2.txt"), 281);
        assert_eq!(trebuchet2("src/d1/input.txt"), 55260);
    }
}