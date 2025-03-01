use std::fs;

#[derive(Debug, Clone)]
struct Coords {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone)]
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

/// found the algo on reddit: https://old.reddit.com/r/adventofcode/comments/18q40he/2023_day_24_part_2_a_straightforward_nonsolver/
fn gaussian_elimination(mut a: Vec<Vec<f64>>) -> Vec<f64> {
    let n = a.len();

    // Forward elimination
    for i in 0..n {
        // Find pivot
        let mut max_row = i;
        for j in i + 1..n {
            if a[j][i].abs() > a[max_row][i].abs() {
                max_row = j;
            }
        }

        if max_row != i {
            a.swap(i, max_row);
        }

        // Scale row
        let pivot = a[i][i];
        if pivot.abs() < 1e-10 {
            println!("Warning: Near-zero pivot encountered");
            continue;
        }

        for j in i..=n {
            a[i][j] /= pivot;
        }

        // Eliminate below
        for j in i + 1..n {
            let factor = a[j][i];
            for k in i..=n {
                a[j][k] -= factor * a[i][k];
            }
        }
    }

    // Back substitution
    for i in (0..n).rev() {
        for j in 0..i {
            let factor = a[j][i];
            a[j][n] -= factor * a[i][n];
            a[j][i] = 0.0;
        }
    }

    // Extract solution
    (0..n).map(|i| a[i][n]).collect()
}

pub fn hail2(file_path: &str) -> usize {
    let stones = parse_lines(file_path);

    // Create equations based on cross product relationships
    let mut matrix_xy = Vec::new();
    let mut matrix_z = Vec::new();

    // Use the first 5 stones
    for i in 0..5 {
        let s = &stones[i];

        // For x-y system
        matrix_xy.push(vec![
            -s.velocity.y,
            s.velocity.x,
            s.position.y,
            -s.position.x,
            s.position.y * s.velocity.x - s.position.x * s.velocity.y,
        ]);

        // For z-y system (matching the Ruby code's approach)
        matrix_z.push(vec![
            -s.velocity.y,
            s.velocity.z,
            s.position.y,
            -s.position.z,
            s.position.y * s.velocity.z - s.position.z * s.velocity.y,
        ]);
    }

    // Create differential equations (subtract last row from each row)
    let ref_xy = matrix_xy.pop().unwrap();
    let ref_z = matrix_z.pop().unwrap();

    let mut eq_xy = Vec::new();
    let mut eq_z = Vec::new();

    for i in 0..4 {
        let mut row_xy = Vec::new();
        let mut row_z = Vec::new();

        for j in 0..4 {
            row_xy.push(matrix_xy[i][j] - ref_xy[j]);
            row_z.push(matrix_z[i][j] - ref_z[j]);
        }
        row_xy.push(matrix_xy[i][4] - ref_xy[4]); // constant term
        row_z.push(matrix_z[i][4] - ref_z[4]); // constant term

        eq_xy.push(row_xy);
        eq_z.push(row_z);
    }

    // Solve the systems
    let xy_solution = gaussian_elimination(eq_xy);
    let z_solution = gaussian_elimination(eq_z);

    // The rock's initial position
    let rx = xy_solution[0];
    let ry = xy_solution[1];
    let rz = z_solution[0];

    println!("Rock position: ({}, {}, {})", rx, ry, rz);

    // Sum the coordinates, rounded to nearest integer
    (rx.round() + ry.round() + rz.round()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d24", "p1", hail1);
    }

    #[test]
    fn p2() {
        check_results("d24", "p2", hail2);
    }
}
