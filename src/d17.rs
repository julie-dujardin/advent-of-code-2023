use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<usize>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut lines = Vec::new();
    for line in file.lines() {
        lines.push(
            line.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect(),
        );
    }
    lines
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Move {
    x: isize,
    y: isize,
    entered_from: Direction,
    acc_cost: usize,
    straight_count: usize,
}

// Make Move comparable for the priority queue
impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering because BinaryHeap is a max-heap
        other.acc_cost.cmp(&self.acc_cost)
    }
}
impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Move {}
impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.acc_cost == other.acc_cost
    }
}

fn get_new_coords(x: isize, y: isize, direction: &Direction) -> (isize, isize) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Right => (x + 1, y),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
    }
}

/// Get the next real direction from the current real direction (entered_from), when changed by the new direction
/// Up -> no change
/// Left -> -90 degrees
/// Right -> +90 degrees
fn change_direction(entered_from: &Direction, new_direction: Direction) -> Direction {
    match new_direction {
        Direction::Up => match entered_from {
            Direction::Up => Direction::Up,
            Direction::Right => Direction::Right,
            Direction::Down => Direction::Down,
            Direction::Left => Direction::Left,
        },
        Direction::Right => match entered_from {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        },
        Direction::Left => match entered_from {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        },
        _ => panic!(),
    }
}

fn get_move(
    curr_move: &Move,
    new_direction: Direction,
    acc_cost: usize,
    straight_count: usize,
) -> Move {
    let new_entered_from = change_direction(&curr_move.entered_from, new_direction);
    let (new_x, new_y) = get_new_coords(curr_move.x, curr_move.y, &new_entered_from);
    Move {
        x: new_x,
        y: new_y,
        entered_from: new_entered_from,
        acc_cost,
        straight_count,
    }
}

pub fn crucible1(file_path: &str) -> usize {
    let map = parse_file(file_path);
    let mut costs = vec![vec![vec![vec![None; 4]; 4]; map[0].len()]; map.len()];

    let mut moves = BinaryHeap::new();
    // First move is special because we don't pay the cost to enter, simulate it & increment the straight count
    moves.push(Move {
        x: 1,
        y: 0,
        entered_from: Direction::Right,
        acc_cost: 0,
        straight_count: 1,
    });
    moves.push(Move {
        x: 0,
        y: 1,
        entered_from: Direction::Down,
        acc_cost: 0,
        straight_count: 1,
    });

    while !moves.is_empty() {
        let curr_move = moves.pop().unwrap();

        if curr_move.x < 0
            || curr_move.y < 0
            || (curr_move.x as usize) >= map[0].len()
            || (curr_move.y as usize) >= map.len()
        {
            // Move is out of map
            continue;
        }

        let (cx, cy) = (curr_move.x as usize, curr_move.y as usize);

        let acc_cost = curr_move.acc_cost + map[cy][cx];
        let direction_idx = match curr_move.entered_from {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        };
        if costs[cy][cx][direction_idx][curr_move.straight_count].is_none()
            || acc_cost < costs[cy][cx][direction_idx][curr_move.straight_count].unwrap()
        {
            // If this is the best or first way to get to this tile, save the new cost.
            costs[cy][cx][direction_idx][curr_move.straight_count] = Some(acc_cost)
        } else {
            // Otherwise, keep going.
            continue;
        }

        if curr_move.straight_count < 3 {
            moves.push(get_move(
                &curr_move,
                Direction::Up,
                acc_cost,
                curr_move.straight_count + 1,
            ));
        }
        moves.push(get_move(&curr_move, Direction::Left, acc_cost, 1));
        moves.push(get_move(&curr_move, Direction::Right, acc_cost, 1));
    }

    let mut min_cost = None;
    for dir in 0..4 {
        for straight in 0..4 {
            if let Some(cost) = costs[map.len() - 1][map[0].len() - 1][dir][straight] {
                min_cost = Some(min_cost.map_or(cost, |m: usize| m.min(cost)));
            }
        }
    }
    min_cost.unwrap()
}

// pub fn crucible2(file_path: &str) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d17", "p1", crucible1);
    }

    // #[test]
    // fn p2() {
    //     check_results("d17", "p2", crucible2);
    // }
}
