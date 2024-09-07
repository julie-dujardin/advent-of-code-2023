use std::fs;
use std::path::absolute;

fn parse_file(file_path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut map = Vec::new();
    for line in file.lines() {
        map.push(line.chars().collect());
    }
    map
}

fn get_galaxies_positions(map: Vec<Vec<char>>) -> Vec<(isize, isize)> {
    let mut x_modifiers = Vec::new();
    for x in 0..map.first().unwrap().len() {
        let mut found_galaxy = false;
        for y in 0..map.len() {
            if map[y][x] == '#' {
                found_galaxy = true;
                break
            }
        }
        if !found_galaxy {
            x_modifiers.push(x);
        }
    }

    let mut galaxies = Vec::new();
    let mut y_modifier = 0;
    for (y, line) in map.iter().enumerate() {
        let mut found_galaxy = false;
        let mut x_modifier = 0;
        for (x, tile) in line.iter().enumerate() {
            if x_modifier < x_modifiers.iter().len() && x > x_modifiers[x_modifier] {
                x_modifier += 1;
            }
            if *tile == '#' {
                galaxies.push(((x + x_modifier) as isize, (y + y_modifier) as isize));
                found_galaxy = true;
            }
        }
        if !found_galaxy {
            y_modifier += 1;
        }
    }

    galaxies
}

fn expansion1(file_path: &str) -> isize {
    let map = parse_file(file_path);
    let galaxies = get_galaxies_positions(map);
    let mut distance_sum = 0;
    let mut couple_count = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for target_galaxy_idx in i+1..galaxies.len() {
            let target_galaxy = galaxies[target_galaxy_idx];
            distance_sum += (target_galaxy.0 - galaxy.0).abs() + (target_galaxy.1 - galaxy.1).abs();
            couple_count += 1;
        }
    }

    distance_sum
}

fn expansion2(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(expansion1("src/d11/input_test1.txt"), 374); // provided test
        assert_eq!(expansion1("src/d11/input.txt"), 9639160);
    }

    // #[test]
    // fn p2() {
    //     assert_eq!(expansion2("src/d11/input_test1.txt"), 1); // provided test
    //     assert_eq!(expansion2("src/d11/input.txt"), 1);
    // }
}