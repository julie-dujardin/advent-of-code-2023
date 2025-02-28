use std::fs;

#[derive(Debug)]
struct Coords {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Hail {
    position: Coords,
    velocity: Coords,
}

fn split_coords(coords: &str) -> Coords {
    let coords_split = coords
        .split(", ")
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    Coords {
        x: coords_split[0],
        y: coords_split[1],
        z: coords_split[2],
    }
}

fn parse_lines(file_path: &str) -> Vec<Hail> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut lines = Vec::new();
    for line in file.lines() {
        let (position_raw, velocity_raw) = line.split_once(" @ ").unwrap();
        lines.push(Hail {
            position: split_coords(position_raw),
            velocity: split_coords(velocity_raw),
        });
    }
    lines
}

fn check_collision(hail0: &Hail, hail1: &Hail) -> bool {
    false
}

pub fn hail1(file_path: &str) -> usize {
    let lines = parse_lines(file_path);

    let mut collision_count = 0;
    for (i, line1) in lines.iter().enumerate() {
        for line2 in lines[i + 1..].iter() {
            if check_collision(line1, line2) {
                collision_count += 1;
            }
        }
    }

    collision_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    // #[test]
    // fn p1() {
    //     check_results("d24", "p1", hail1);
    // }

    // #[test]
    // fn p2() {
    //     check_results("d24", "p2", hail2);
    // }
}
