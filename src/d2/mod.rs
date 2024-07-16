use std::cmp::max;
use std::collections::HashMap;
use std::fs;

fn line_min_counts(line_split: Vec<&str>) -> HashMap<&str, i32> {
    let mut maximums = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    for reveals in line_split[1].split("; ") {
        for reveal in reveals.split(", ") {
            let reveal_split: Vec<&str> = reveal.split(' ').collect();
            maximums.insert(
                reveal_split[1],
                max(reveal_split[0].parse().unwrap(), maximums[reveal_split[1]]),
            );
        }
    }
    // TODO logging
    // println!("game {game_number} maximums: {maximums:?}");
    maximums
}

fn cubes1(file_path: &str) -> i32 {
    let mut sum = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let line_split: Vec<&str> = line.split(": ").collect();
        let game_number: i32 = line_split[0][5..].parse().unwrap();

        let maximums = line_min_counts(line_split);
        if maximums["red"] <= 12 && maximums["green"] <= 13 && maximums["blue"] <= 14 {
            sum += game_number;
        }
    }
    println!("The sum of possible IDs is {sum}");
    sum
}

fn cubes2(file_path: &str) -> i32 {
    let mut sum = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let line_split: Vec<&str> = line.split(": ").collect();

        let maximums = line_min_counts(line_split);
        sum += maximums["red"] * maximums["green"] * maximums["blue"]
    }
    println!("The sum of powers is {sum}");
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(cubes1("src/d2/input_test1.txt"), 8);
        assert_eq!(cubes1("src/d2/input.txt"), 2406);
    }
    #[test]
    fn p2() {
        assert_eq!(cubes2("src/d2/input_test2.txt"), 2286);
        assert_eq!(cubes2("src/d2/input.txt"), 78375);
    }
}
