use std::collections::{HashMap, HashSet};
use std::fs;

struct Brick {
    coords_start: (usize, usize, usize),
    coords_end: (usize, usize, usize),
    above: HashSet<usize>,
    under: HashSet<usize>,
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
            above: HashSet::new(),
            under: HashSet::new(),
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
        let mut above_bricks = HashMap::new();

        for (i, brick) in bricks.iter_mut().enumerate() {
            let mut z = brick.coords_start.2;

            while z >= 2 && brick.under.len() == 0 {
                z -= 1;

                for x in brick.coords_start.0..brick.coords_end.0 + 1 {
                    for y in brick.coords_start.1..brick.coords_end.1 + 1 {
                        if let Some(under_brick) = self.get(x, y, z) {
                            brick.under.insert(under_brick);
                            above_bricks
                                .entry(under_brick)
                                .and_modify(|set: &mut HashSet<usize>| {
                                    set.insert(i);
                                })
                                .or_insert_with(|| {
                                    let mut set = HashSet::new();
                                    set.insert(i);
                                    set
                                });

                            // bricks[under_brick].above.insert(i);
                        }
                    }
                }
            }

            if z < brick.coords_start.2 {
                z += 1; // got a collision, go back up one layer

                self.pop_brick(&brick);
                let delta = brick.coords_start.2 - z;
                brick.coords_start.2 -= delta;
                brick.coords_end.2 -= delta;
                self.push_brick(&brick, i);
            }
        }

        for (under_brick, above_bricks) in above_bricks {
            bricks[under_brick].above = above_bricks;
        }
    }
}

fn parse_n_gravity(file_path: &str) -> (Map, Vec<Brick>) {
    let mut bricks = parse_file(file_path);
    let mut map = Map {
        storage: Default::default(),
        max_z: 0,
    };
    map.push_bricks(&bricks);
    map.gravity(&mut bricks);
    (map, bricks)
}

pub fn bricks1(file_path: &str) -> usize {
    let (_map, bricks) = parse_n_gravity(file_path);

    let mut can_remove: HashSet<usize> = HashSet::from_iter(0..bricks.len());
    for brick in bricks.iter() {
        if brick.under.len() == 1 {
            can_remove.remove(&brick.under.iter().next().unwrap());
        }
    }

    can_remove.len()
}

fn count_children_rec(
    node: usize,
    bricks_children: &HashMap<usize, HashSet<usize>>,
    cache: &mut HashMap<usize, HashSet<usize>>,
) -> HashSet<usize> {
    // Check if result is already in cache
    if let Some(cached) = cache.get(&node) {
        return cached.clone();
    }

    // TODO check if the node has parents that haven't been visited

    let mut nodes = HashSet::new();
    for child in bricks_children.get(&node).unwrap() {
        nodes.insert(*child);
        nodes.extend(count_children_rec(*child, bricks_children, cache));
    }

    // Store result in cache before returning
    cache.insert(node, nodes.clone());
    nodes
}

// pub fn bricks2(file_path: &str) -> usize {
//     let (map, bricks) = parse_n_gravity(file_path);
//
//     let bricks_ancestors = get_ancestors(&map, &bricks);
//     // let bricks_children = get_children(map, &bricks);
//
//     let mut counts = HashMap::new();
//     // for (_higher, lower) in bricks_ancestors.iter() {
//     //     if lower.len() == 1 {
//     //         let start_node = lower.iter().next().unwrap();
//     //         counts.entry(start_node).or_insert(
//     //             count_children_rec(*start_node, &bricks_children, &mut HashMap::new()).len(),
//     //         );
//     //     }
//     // }
//
//     counts.values().sum()
// }

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

    // #[test]
    // fn p2() {
    //     check_results("d22", "p2", bricks2);
    // }
}
