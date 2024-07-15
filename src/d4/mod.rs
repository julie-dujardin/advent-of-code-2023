use std::collections::HashSet;
use std::fs;

fn numbers_str_to_vec(s: &str) -> HashSet<i32> {
    let mut nums = HashSet::new();
    for number_s in s.split(" ").filter(|&x| !x.is_empty()) {
        nums.insert(number_s.parse().unwrap());
    }
    nums
}

fn scratchcards1(file_path: &str, offset: usize) -> i32 {
    let mut sum = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let mut number_groups = line[offset..].split(" | ");

        let winning_nums = numbers_str_to_vec(number_groups.nth(0).unwrap());
        let have_nums = numbers_str_to_vec(number_groups.nth(0).unwrap());

        let card_win_count = have_nums.intersection(&winning_nums).count();
        if card_win_count > 0{
            sum += 2_i32.pow((card_win_count - 1) as u32);
        }
    }

    println!("The scratchcards are worth {sum}");
    sum
}


mod tests {
    use super::*;

    #[test]
    fn n2vec() {
        assert_eq!(numbers_str_to_vec(" 51 58  5 54 "), HashSet::from([51, 58, 5, 54]));
    }

    #[test]
    fn p1() {
        assert_eq!(scratchcards1("src/d4/input_test1.txt", 7), 13); // provided test
        assert_eq!(scratchcards1("src/d4/input.txt", 9), 27454);
    }
}
