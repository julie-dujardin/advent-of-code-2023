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
        for y in 0..map.len() {
            if map[y][x] == '#' {
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

fn sum_distances(galaxies: Vec<(isize, isize)>) -> isize {
    let mut distance_sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for target_galaxy_idx in i + 1..galaxies.len() {
            let target_galaxy = galaxies[target_galaxy_idx];
            distance_sum += (target_galaxy.0 - galaxy.0).abs() + (target_galaxy.1 - galaxy.1).abs();
        }
    }

    distance_sum
}

fn expansion(file_path: &str, expansion: usize) -> isize {
    let map = parse_file(file_path);
    let galaxies = get_galaxies_positions(map, expansion);
    sum_distances(galaxies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(expansion("src/d11/input_test1.txt", 1), 374); // provided test
        assert_eq!(expansion("src/d11/input.txt", 1), 9639160);
    }

    #[test]
    fn p2() {
        // There's an off-by-one error somewhere ig
        assert_eq!(expansion("src/d11/input_test1.txt", 9), 1030); // provided test
        assert_eq!(expansion("src/d11/input_test1.txt", 99), 8410); // provided test
        assert_eq!(expansion("src/d11/input.txt", 999999), 752936133304);
    }
}
