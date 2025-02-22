use std::collections::{HashMap, HashSet};
use std::fs;

struct Brick {
    coords_start: (usize, usize, usize),
    coords_end: (usize, usize, usize),
}

fn parse_file(file_path: &str) -> Vec<Brick> {
    let file = fs::read_to_string(file_path).unwrap();

    let mut bricks = Vec::new();

    for line in file.lines() {
        let (raw_start, raw_end) = line.split_once('~').unwrap();
        let start = raw_start
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let end = raw_end
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        bricks.push(Brick {
            coords_start: (start[0], start[1], start[2]),
            coords_end: (end[0], end[1], end[2]),
        })
    }

    bricks.sort_by(|x1, x2| x1.coords_start.2.cmp(&x2.coords_start.2));
    bricks
}

struct Map {
    // storage[x][y][z] = block id
    // Points with no brick are omitted
    storage: HashMap<usize, HashMap<usize, HashMap<usize, usize>>>,
    max_z: usize,
}

impl Map {
    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<usize> {
        Some(*self.storage.get(&x)?.get(&y)?.get(&z)?)
    }

    pub fn pop(&mut self, x: usize, y: usize, z: usize) -> Option<usize> {
        Some(self.storage.get_mut(&x)?.get_mut(&y)?.remove(&z)?)
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: usize) {
        self.storage
            .entry(x)
            .or_insert_with(HashMap::new)
            .entry(y)
            .or_insert_with(HashMap::new)
            .insert(z, value);
        self.max_z = self.max_z.max(z);
    }

    fn push_brick(&mut self, brick: &Brick, brick_id: usize) {
        for x in brick.coords_start.0..brick.coords_end.0 + 1 {
            for y in brick.coords_start.1..brick.coords_end.1 + 1 {
                for z in brick.coords_start.2..brick.coords_end.2 + 1 {
                    self.set(x, y, z, brick_id)
                }
            }
        }
    }

    fn pop_brick(&mut self, brick: &Brick) {
        for x in brick.coords_start.0..brick.coords_end.0 + 1 {
            for y in brick.coords_start.1..brick.coords_end.1 + 1 {
                for z in brick.coords_start.2..brick.coords_end.2 + 1 {
                    self.pop(x, y, z);
                }
            }
        }
    }

    pub fn push_bricks(&mut self, bricks: &Vec<Brick>) {
        for (i, brick) in bricks.iter().enumerate() {
            self.push_brick(brick, i)
        }
    }

    pub fn gravity(&mut self, bricks: &mut Vec<Brick>) {
        for (i, brick) in bricks.iter_mut().enumerate() {
            let mut z = brick.coords_start.2;

            'outer: while z > 1 {
                for x in brick.coords_start.0..brick.coords_end.0 + 1 {
                    for y in brick.coords_start.1..brick.coords_end.1 + 1 {
                        if self.get(x, y, z - 1).is_some() {
                            break 'outer;
                        }
                    }
                }

                z -= 1;
            }

            if z < brick.coords_start.2 {
                self.pop_brick(&brick);
                let delta = brick.coords_start.2 - z;
                brick.coords_start.2 -= delta;
                brick.coords_end.2 -= delta;
                self.push_brick(&brick, i);
            }
        }
    }
}

pub fn bricks1(file_path: &str) -> usize {
    let mut bricks = parse_file(file_path);
    let mut map = Map {
        storage: Default::default(),
        max_z: 0,
    };
    map.push_bricks(&bricks);
    map.gravity(&mut bricks);

    let mut bricks_ancestors = HashMap::new();
    // high -> [low]
    for (i, _) in bricks.iter().enumerate() {
        bricks_ancestors.insert(i, HashSet::new());
    }

    for (lower, brick) in bricks.iter().enumerate() {
        for x in brick.coords_start.0..brick.coords_end.0 + 1 {
            for y in brick.coords_start.1..brick.coords_end.1 + 1 {
                // If we find a child above this parent, the parent to this child's parent list
                if let Some(higher) = map.get(x, y, brick.coords_end.2 + 1) {
                    bricks_ancestors.get_mut(&higher).unwrap().insert(lower);
                }
            }
        }
    }

    let mut can_remove: HashSet<usize> = HashSet::from_iter(0..bricks.len());
    for (_higher, lower) in bricks_ancestors.iter() {
        if lower.len() == 1 {
            can_remove.remove(&lower.iter().next().unwrap());
        }
    }

    can_remove.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn test_parse_file() {
        let mut bricks = parse_file("test-data/d22/input_test1.txt");
        let mut map = Map {
            storage: Default::default(),
            max_z: 0,
        };
        map.push_bricks(&bricks);

        assert_eq!(map.get(2, 2, 5).unwrap(), 4);
        assert_eq!(map.get(1, 1, 6).unwrap(), 5);

        map.gravity(&mut bricks);

        assert_eq!(map.get(2, 2, 3).unwrap(), 4);
        assert_eq!(map.get(1, 1, 4).unwrap(), 5);
    }

    #[test]
    fn p1() {
        check_results("d22", "p1", bricks1);
    }
}
