use phf::phf_map;
use std::fs;

// (delta to get into the tile, delta to get out of it)
// reverse the order, and multiply them by -1 to get the reverse direction
static TILES: phf::Map<char, ((i32, i32), (i32, i32))> = phf_map! {
    '|' => ((0, 1), (0, 1)),
    '-' => ((-1, 0), (-1, 0)),
    'L' => ((-1, 0), (0, -1)),
    'J' => ((1, 0), (0, -1)),
    '7' => ((0, -1), (-1, 0)),
    'F' => ((0, -1), (1, 0)),
    '.' => ((0, 0), (0, 0)),
};
// up, right, below, left
static SURROUNDING_COORDS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse_file(file_path: &str) -> Maze {
    let file = fs::read_to_string(file_path).unwrap();
    let mut maze = Maze {
        lines: Vec::new(),
        start_pos: (0, 0),
    };
    for (y, line) in file.lines().enumerate() {
        let start_opt = line.find('S');
        if let Some(start_x) = start_opt {
            maze.start_pos = (start_x, y);
        }
        maze.lines.push(line.chars().collect());
    }
    maze
}

struct Maze {
    start_pos: (usize, usize),
    lines: Vec<Vec<char>>,
}

impl Maze {
    pub fn get(&self, x: usize, y: usize) -> char {
        self.lines[y][x]
    }
}

fn add_pos(start: (usize, usize), delta: (i32, i32)) -> (usize, usize) {
    (
        (start.0 as i32 + delta.0) as usize,
        (start.1 as i32 + delta.1) as usize,
    )
}

fn get_first_move(maze: &Maze) -> ((usize, usize), bool, (i32, i32)) {
    for delta in SURROUNDING_COORDS {
        let check_pos = add_pos(maze.start_pos, delta);
        // overflow is a feature
        if check_pos.0 < maze.lines.first().unwrap().len() && check_pos.1 < maze.lines.len() {
            let tile = &maze.get(check_pos.0, check_pos.1);
            if TILES[tile].0 == delta {
                return (check_pos, false, delta);
            }
            if (-TILES[tile].1 .0, -TILES[tile].1 .1) == delta {
                return (check_pos, true, delta);
            }
        }
    }
    panic!("Didn't find a first move!!!")
}

fn maze1(file_path: &str) -> isize {
    let maze = parse_file(file_path);
    let (mut curr_pos, mut reverse, _) = get_first_move(&maze);
    let mut curr_tile = maze.get(curr_pos.0, curr_pos.1);
    let mut maze_len = 0;

    loop {
        maze_len += 1;

        let next_delta = if reverse {
            (-TILES[&curr_tile].0 .0, -TILES[&curr_tile].0 .1)
        } else {
            TILES[&curr_tile].1
        };

        curr_pos = add_pos(curr_pos, next_delta);
        curr_tile = maze.get(curr_pos.0, curr_pos.1);
        if curr_tile == 'S' {
            break;
        }
        reverse = TILES[&curr_tile].0 != next_delta
    }

    (maze_len + 1) / 2
}

fn clean_maze(maze: Maze) -> Maze {
    // Remove all tiles that aren't part of the loop
    // Also, replace the start tile with the real tile it replaced

    let (mut curr_pos, reverse_start, delta_start) = get_first_move(&maze);
    let mut reverse = reverse_start;
    let mut delta_end = (0, 0);
    let mut curr_tile = maze.get(curr_pos.0, curr_pos.1);
    let mut maze_clean = Maze {
        lines: vec![vec!['.'; maze.lines.first().unwrap().len()]; maze.lines.len()],
        start_pos: maze.start_pos,
    };
    maze_clean.lines[curr_pos.1][curr_pos.0] = curr_tile;

    loop {
        let next_delta = if reverse {
            (-TILES[&curr_tile].0 .0, -TILES[&curr_tile].0 .1)
        } else {
            TILES[&curr_tile].1
        };

        curr_pos = add_pos(curr_pos, next_delta);
        curr_tile = maze.get(curr_pos.0, curr_pos.1);
        if curr_tile == 'S' {
            delta_end = next_delta;
            break;
        }
        maze_clean.lines[curr_pos.1][curr_pos.0] = curr_tile;
        reverse = TILES[&curr_tile].0 != next_delta
    }

    let mut delta_first = (delta_end, delta_start);
    loop {
        for (tile, delta) in TILES.entries() {
            if *delta == delta_first {
                maze_clean.lines[maze_clean.start_pos.1][maze_clean.start_pos.0] = *tile;
                return maze_clean;
            }
        }
        // try reverse too
        delta_first = (
            (-delta_start.0, -delta_start.1),
            (-delta_end.0, -delta_end.1),
        );
    }
}

fn maze2(file_path: &str) -> isize {
    let maze = parse_file(file_path);
    let maze_clean = clean_maze(maze);

    let mut enclosed_count = 0;

    // Cast a ray through the top of each line
    // Any tile we visit after colliding with the loop an odd number of time is enclosed
    // 7, -, and F don't count because they don't go all the way to the top
    for line in maze_clean.lines {
        let mut collision_count = 0;
        for tile in line {
            if let 'J' | '|' | 'L' = tile {
                collision_count += 1;
            } else if '.' == tile && collision_count % 2 != 0 {
                enclosed_count += 1;
            }
        }
    }

    enclosed_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(maze1("src/d10/input_test1_simple.txt"), 4); // provided test
        assert_eq!(maze1("src/d10/input_test1.txt"), 4); // provided test
        assert_eq!(maze1("src/d10/input_test2_simple.txt"), 8); // provided test
        assert_eq!(maze1("src/d10/input_test2.txt"), 8); // provided test
        assert_eq!(maze1("src/d10/input.txt"), 6947);
    }

    #[test]
    fn p2() {
        assert_eq!(maze2("src/d10/input_test3.txt"), 4); // provided test
        assert_eq!(maze2("src/d10/input_test4.txt"), 8); // provided test
        assert_eq!(maze2("src/d10/input_test5.txt"), 10); // provided test
        assert_eq!(maze2("src/d10/input.txt"), 273);
    }
}
