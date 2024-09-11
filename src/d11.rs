use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut map = Vec::new();
    for line in file.lines() {
        map.push(line.chars().collect());
    }
    map
}

fn get_galaxies_positions(map: Vec<Vec<char>>, expansion: usize) -> Vec<(isize, isize)> {
    // It's probably possible to do all empty column/line check & galaxy position adjustment
    // with a single loop over the map, but it's getting too late for this kind of stuff
    let mut x_modifiers = Vec::new();
    // Get a list of all empty columns, which should be expanded
    for x in 0..map.first().unwrap().len() {
        let mut found_galaxy = false;
        for line in &map {
            if line[x] == '#' {
                found_galaxy = true;
                break;
            }
        }
        if !found_galaxy {
            x_modifiers.push(x);
        }
    }

    let mut galaxies = Vec::new();
    // Line expansion is calculated during the second and last time we go over the map
    let mut y_modifier = 0;
    for (y, line) in map.iter().enumerate() {
        let mut found_galaxy = false;
        let mut x_modifier = 0;
        for (x, tile) in line.iter().enumerate() {
            // Check if we've gotten beyond an empty column & adjust the x coord accordingly
            if x_modifier / expansion < x_modifiers.iter().len()
                && x > x_modifiers[x_modifier / expansion]
            {
                x_modifier += expansion;
            }
            if *tile == '#' {
                galaxies.push(((x + x_modifier) as isize, (y + y_modifier) as isize));
                found_galaxy = true;
            }
        }
        if !found_galaxy {
            y_modifier += expansion;
        }
    }

    galaxies
}

fn sum_distances(galaxies: Vec<(isize, isize)>) -> usize {
    let mut distance_sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for target_galaxy in galaxies.iter().skip(i + 1) {
            distance_sum += (target_galaxy.0 - galaxy.0).abs() + (target_galaxy.1 - galaxy.1).abs();
        }
    }

    println!("expansion:{}", distance_sum);
    distance_sum as usize
}

pub fn expansion(file_path: &str, expansion: usize) -> usize {
    let map = parse_file(file_path);
    let galaxies = get_galaxies_positions(map, expansion);
    sum_distances(galaxies)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::load_results;

    #[test]
    fn p1() {
        let expected_p1 = load_results("d11", "p1");
        assert_eq!(
            expansion("test-data/d11/input_test1.txt", 1),
            expected_p1["input_test1"]
        );
        assert_eq!(
            expansion("test-data/d11/input.txt", 1),
            expected_p1["input"]
        );
    }

    #[test]
    fn p2() {
        let expected_p2 = load_results("d11", "p2");
        // There's an off-by-one error somewhere ig
        assert_eq!(
            expansion("test-data/d11/input_test1.txt", 9),
            expected_p2["input_test1"]
        );
        assert_eq!(
            expansion("test-data/d11/input_test1.txt", 99),
            expected_p2["input_test1_2"]
        );
        assert_eq!(
            expansion("test-data/d11/input.txt", 999999),
            expected_p2["input"]
        );
    }
}
