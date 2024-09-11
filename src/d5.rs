use std::cmp::{max, min};
use std::fs;

fn numbers_str_to_vec(s: &str) -> Vec<isize> {
    let mut nums = Vec::new();
    for number_s in s.split(' ').filter(|&x| !x.is_empty()) {
        nums.push(number_s.parse().unwrap());
    }
    nums
}

pub fn seed1(file_path: &str) -> usize {
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
    min_location as usize
}

pub fn seed2(file_path: &str) -> usize {
    let file = fs::read_to_string(file_path).unwrap();
    let mut groups = file.split("\n\n");

    // Init seeds
    let mut seed_ranges = Vec::new();
    let ranges = numbers_str_to_vec(&groups.next().unwrap()[7..]);
    for i in 0..ranges.len() / 2 {
        seed_ranges.push((ranges[i * 2], ranges[i * 2 + 1]));
    }

    for group in groups {
        let mut lines = group.lines();
        lines.next(); // ignore first line

        let mut new_seed_ranges = Vec::new();
        for mutation_range in lines {
            let numbers = numbers_str_to_vec(mutation_range);

            let mut_src_start = numbers[1];
            let mut_dest_start = numbers[0];
            let mut_len = numbers[2];
            let mut_src_end = mut_src_start + mut_len;

            let mut new_old_ranges = Vec::new();
            let mut full_match_ranges = Vec::new();

            for (i, seed) in seed_ranges.iter_mut().enumerate() {
                let old_start = seed.0;
                let old_len = seed.1;
                let old_end = old_start + old_len;

                // Detect collision between a seed range and a mutation range
                if old_end >= mut_src_start && old_start <= mut_src_end {
                    if old_start < mut_src_start && old_end <= mut_src_end {
                        // partial range match, starts early
                        // shorten the source range before mutating the matching part of the source range & add it to new_seed_ranges
                        seed.1 = mut_src_start - old_start
                    } else if old_end > mut_src_end && old_start <= mut_src_end {
                        // partial match, ends late
                        let new_sr_start = mut_src_end;
                        seed.1 -= new_sr_start - seed.0;
                        seed.0 = mut_src_end;
                    } else if old_start < mut_src_start && old_end > mut_src_end {
                        // source range bigger than match range
                        // The range has to be split
                        seed.1 = mut_src_start - old_start;
                        new_old_ranges.push((mut_src_end, old_len - (mut_src_end - old_start)));
                    } else {
                        // full source range match - drop it after end of loop
                        full_match_ranges.push(i);
                    }
                    // push mutated range
                    let mut_delta = mut_dest_start - mut_src_start;
                    let new_range_start = max(old_start, mut_src_start) + mut_delta;
                    let new_range_end = min(old_end, mut_src_end) + mut_delta;
                    new_seed_ranges.push((new_range_start, new_range_end - new_range_start))
                }
            }
            // drop fully matched range
            for (range_mod, fully_matched_index) in full_match_ranges.iter().enumerate() {
                seed_ranges.remove(fully_matched_index - range_mod);
            }
            seed_ranges.extend(new_old_ranges);
        }
        seed_ranges.extend(new_seed_ranges);
    }
    // Get the smallest start of range
    let mut min_location = -1;
    for (range_start, _) in seed_ranges {
        if min_location == -1 || range_start < min_location {
            min_location = range_start;
        }
    }

    println!("The lowest location number is {min_location}");
    min_location as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::load_results;

    #[test]
    fn p1() {
        let expected_p1 = load_results("d5", "p1");
        assert_eq!(
            seed1("test-data/d5/input_test1.txt"),
            expected_p1["input_test1"]
        );
        assert_eq!(seed1("test-data/d5/input.txt"), expected_p1["input"]);
    }

    #[test]
    fn p2() {
        let expected_p2 = load_results("d5", "p2");
        assert_eq!(
            seed2("test-data/d5/input_test1.txt"),
            expected_p2["input_test1"]
        );
        assert_eq!(seed2("test-data/d5/input.txt"), expected_p2["input"]);
    }
}
