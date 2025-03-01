use std::fs;

#[derive(Debug)]
struct Coords {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Hail {
    position: Coords,
    velocity: Coords,
}

fn split_coords(coords: &str) -> Coords {
    let coords_split = coords
        .split(", ")
        .map(|x| x.trim().parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
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

fn check_collision(hail1: &Hail, hail2: &Hail, min_pos: f64, max_pos: f64) -> bool {
    let determinant = hail1.velocity.x * hail2.velocity.y - hail2.velocity.x * hail1.velocity.y;
    if determinant == 0. {
        return false;
    };

    let intersect_time1 = (((hail2.position.x - hail1.position.x) * hail2.velocity.y)
        - ((hail2.position.y - hail1.position.y) * hail2.velocity.x))
        / determinant;

    let intersect_x1 = hail1.position.x + hail1.velocity.x * intersect_time1;
    let intersect_y1 = hail1.position.y + hail1.velocity.y * intersect_time1;

    let intersect_time2;
    if hail2.velocity.x != 0.0 {
        intersect_time2 = (intersect_x1 - hail2.position.x) / hail2.velocity.x;
    } else {
        intersect_time2 = (intersect_y1 - hail2.position.y) / hail2.velocity.y;
    }

    // No time travel
    if intersect_time1 < 0. || intersect_time2 < 0. {
        return false;
    }

    intersect_x1 >= min_pos
        && intersect_x1 <= max_pos
        && intersect_y1 >= min_pos
        && intersect_y1 <= max_pos
}

pub fn hail1(file_path: &str) -> usize {
    let lines = parse_lines(file_path);

    let check_area = if file_path.contains("_test") {
        (7., 27.)
    } else {
        (200000000000000., 400000000000000.)
    };

    let mut collision_count = 0;
    for (i, line1) in lines.iter().enumerate() {
        for line2 in lines[i + 1..].iter() {
            if check_collision(line1, line2, check_area.0, check_area.1) {
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

    #[test]
    fn p1() {
        check_results("d24", "p1", hail1);
    }

    // #[test]
    // fn p2() {
    //     check_results("d24", "p2", hail2);
    // }
}
