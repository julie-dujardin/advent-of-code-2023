use std::collections::{HashMap, HashSet};
use std::fs;

fn numbers_str_to_hashset(s: &str) -> HashSet<i32> {
    let mut nums = HashSet::new();
    for number_s in s.split(' ').filter(|&x| !x.is_empty()) {
        nums.insert(number_s.parse().unwrap());
    }
    nums
}

fn scratchcards1(file_path: &str, offset: usize) -> i32 {
    let mut sum = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let mut number_groups = line[offset..].split(" | ");

        let winning_nums = numbers_str_to_hashset(number_groups.next().unwrap());
        let have_nums = numbers_str_to_hashset(number_groups.next().unwrap());

        let card_win_count = have_nums.intersection(&winning_nums).count();
        if card_win_count > 0 {
            sum += 2_i32.pow((card_win_count - 1) as u32);
        }
    }

    println!("The scratchcards are worth {sum}");
    sum
}

fn scratchcards2(file_path: &str, offset: usize) -> i32 {
    let mut sum = 0;
    let mut copy_counts: HashMap<usize, i32> = HashMap::new();

    for (i, line) in fs::read_to_string(file_path).unwrap().lines().enumerate() {
        let multiplier = 1 + if copy_counts.get(&i).is_some() {
            copy_counts.get(&i).unwrap()
        } else {
            &0
        };
        sum += multiplier;

        let mut number_groups = line[offset..].split(" | ");

        let winning_nums = numbers_str_to_hashset(number_groups.next().unwrap());
        let have_nums = numbers_str_to_hashset(number_groups.next().unwrap());

        let card_win_count = have_nums.intersection(&winning_nums).count();
        if card_win_count > 0 {
            for j in i + 1..card_win_count + i + 1 {
                copy_counts
                    .entry(j)
                    .and_modify(|e| *e += multiplier)
                    .or_insert(multiplier);
            }
        }
    }

    println!("The scratchcards are worth {sum}");
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n2vec() {
        assert_eq!(
            numbers_str_to_hashset(" 51 58  5 54 "),
            HashSet::from([51, 58, 5, 54])
        );
    }

    #[test]
    fn p1() {
        assert_eq!(scratchcards1("src/d4/input_test1.txt", 7), 13); // provided test
        assert_eq!(scratchcards1("src/d4/input.txt", 9), 27454);
    }

    #[test]
    fn p2() {
        assert_eq!(scratchcards2("src/d4/input_test1.txt", 7), 30); // provided test
        assert_eq!(scratchcards2("src/d4/input.txt", 9), 6857330);
    }
}
