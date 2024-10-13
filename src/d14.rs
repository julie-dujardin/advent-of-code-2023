use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        for (i, char) in line.chars().enumerate(){
            if map.len() <= i {
                map.push(Vec::new());
            }
            (&mut map[i]).push(char)
        }
    }
    map
}

pub fn dishes1(file_path: &str) -> usize {
    let map = parse_file(file_path);

    let mut load = 0;

    for mut column in map {
        let mut back_ptr = 0;
        let mut fwd_ptr = 1;
        while back_ptr < column.len() {
            if column[back_ptr] == 'O' {
                load += column.len() - back_ptr;
                fwd_ptr += 1;
            }
            else if column[back_ptr] == '#' {
                fwd_ptr += 1;
            }
            else {
                while fwd_ptr < column.len() {
                    if column[fwd_ptr] == 'O' {
                        load += column.len() - back_ptr;
                        column[fwd_ptr] = '.';
                        column[back_ptr] = 'O';
                        break
                    }
                    if column[fwd_ptr] == '#' {
                        break
                    }
                    fwd_ptr += 1;
                }
            }

            back_ptr += 1;
        }
    }

    load
}

fn get_load(map: Vec<Vec<char>>) -> usize{
    let mut load = 0;
    for mut column in map {
        let mut back_ptr = 0;
        while back_ptr < column.len() {
            if column[back_ptr] == 'O' {
                load += column.len() - back_ptr;
            }

            back_ptr += 1;
        }
    }
    load
}

pub fn dishes2(file_path: &str) -> usize {
    let map = parse_file(file_path);



    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::load_results;

    #[test]
    fn p1() {
        let expected = load_results("d14", "p1");
        assert_eq!(
            dishes1("test-data/d14/input_test1.txt"),
            expected["input_test1"]
        );
        assert_eq!(dishes1("test-data/d14/input.txt"), expected["input"]);
    }

    #[test]
    fn p2() {
        let expected = load_results("d14", "p2");
        assert_eq!(
            dishes1("test-data/d14/input_test1.txt"),
            expected["input_test1"]
        );
        assert_eq!(dishes1("test-data/d14/input.txt"), expected["input"]);
    }
}