use std::fs;

fn numbers_str_to_vec(s: &str) -> Vec<isize> {
    let mut nums = Vec::new();
    for number_s in s.split(' ').filter(|&x| !x.is_empty()) {
        nums.push(number_s.parse().unwrap());
    }
    nums
}

fn seed1(file_path: &str) -> isize {
    let file = fs::read_to_string(file_path).unwrap();
    let mut groups = file.split("\n\n");

    // Init seeds
    let mut seeds = numbers_str_to_vec(&groups.next().unwrap()[7..]);

    for group in groups {
        let mut lines = group.lines();
        lines.next(); // ignore first line
        let mut moved: Vec<usize> = Vec::new();

        for range in lines {
            let numbers = numbers_str_to_vec(range);

            let src_start = numbers[1];
            let dest_start = numbers[0];
            let range_len = numbers[2];

            for i in 0..seeds.len() {
                // If the seed matches the current range & we did not move it during this group, move it
                if !moved.contains(&i) && seeds[i] >= src_start && seeds[i] <= src_start + range_len
                {
                    seeds[i] += dest_start - src_start;
                    moved.push(i);
                }
            }
        }
    }

    let min_location = *seeds.iter().min().unwrap();
    println!("The lowest location number is {min_location}");
    min_location
}

fn seed2_bruteforce(file_path: &str) -> isize {
    let file = fs::read_to_string(file_path).unwrap();
    let mut groups = file.split("\n\n");

    // Init seeds
    let mut seeds = Vec::new();
    let ranges = numbers_str_to_vec(&groups.next().unwrap()[7..]);
    for i in 0..ranges.len()/2  {
        for j in ranges[i*2]..ranges[i]+ranges[i+1]{
            seeds.push(j);
        }
    }

    for group in groups {
        let mut lines = group.lines();
        lines.next(); // ignore first line
        let mut moved: Vec<usize> = Vec::new();

        for range in lines {
            let numbers = numbers_str_to_vec(range);

            let src_start = numbers[1];
            let dest_start = numbers[0];
            let range_len = numbers[2];

            for i in 0..seeds.len() {
                // If the seed matches the current range & we did not move it during this group, move it
                if !moved.contains(&i) && seeds[i] >= src_start && seeds[i] <= src_start + range_len
                {
                    seeds[i] += dest_start - src_start;
                    moved.push(i);
                }
            }
        }
    }

    let min_location = *seeds.iter().min().unwrap();
    println!("The lowest location number is {min_location}");
    min_location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(seed1("src/d5/input_test1.txt"), 35); // provided test
        assert_eq!(seed1("src/d5/input.txt"), 175622908);
    }

    #[test]
    fn p2_bruteforce() {
        assert_eq!(seed2_bruteforce("src/d5/input_test1.txt"), 46); // provided test
        // seed2_bruteforce("src/d5/input.txt");
    }
}
