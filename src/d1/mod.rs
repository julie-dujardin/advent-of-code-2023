use std::collections::HashMap;
use std::fs;

pub fn test_trebuchet() {
    assert_eq!(trebuchet1(), 55123);
    assert_eq!(trebuchet2(), 55260);
}

fn trebuchet1() -> i32 {
    let mut sum= 0;
    // TODO line buffering
    // TODO line multithreading
    for line in fs::read_to_string("src/d1/input.txt").unwrap().lines(){
        let mut number = String::new();
        let digits: Vec<&str> = line.matches(char::is_numeric).collect();
        number.push_str(digits[0]);
        number.push_str(digits.last().unwrap());
        sum += number.parse::<i32>().unwrap()
    }
    println!("The calibration value is {sum}");
    return sum;
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
    return (li, ln);
}

fn trebuchet2() -> i32 {
    let mut sum= 0;
    for line in fs::read_to_string("src/d1/input.txt").unwrap().lines(){
        let mut number = String::new();

        let (li, ln) = find_text_digit(&line, false);
        for (i, a) in line.chars().enumerate() {
            if i >= li {
                break
            }
            if a.is_digit(10) {
                number.push(a);
                break;
            }
        }
        if number.len() == 0 {
            number.push(ln)
        }

        let (ri, rn) = find_text_digit(&line, true);
        for (i, a) in line.chars().rev().enumerate() {
            if (line.len() - i) <= ri {
                break
            }
            if a.is_digit(10) {
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
    return sum;
}