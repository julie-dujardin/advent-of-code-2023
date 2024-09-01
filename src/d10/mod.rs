use phf::phf_map;
use std::fs;

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
        maze.lines.push(line.to_string());
    }
    maze
}

struct Maze {
    start_pos: (usize, usize),
    lines: Vec<String>,
}

impl Maze {
    pub fn get(&self, x: usize, y: usize) -> char {
        if let Some(tile) = self.lines[y].chars().nth(x) {
            return tile;
        }
        panic!("Out of bounds")
    }
}

fn add_pos(start: (usize, usize), delta: (i32, i32)) -> (usize, usize) {
    (
        (start.0 as i32 + delta.0) as usize,
        (start.1 as i32 + delta.1) as usize,
    )
}

fn get_first_move(maze: &Maze) -> ((usize, usize), bool) {
    for delta in SURROUNDING_COORDS {
        let check_pos = add_pos(maze.start_pos, delta);
        let tile = &maze.get(check_pos.0, check_pos.1);
        if TILES[tile].0 == delta {
            return (check_pos, false);
        }
        if (-TILES[tile].1 .0, -TILES[tile].1 .1) == delta {
            return (check_pos, true);
        }
    }
    panic!("Didn't find a first move!!!")
}

fn maze1(file_path: &str) -> isize {
    let maze = parse_file(file_path);
    let (mut curr_pos, mut reverse) = get_first_move(&maze);
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

fn maze2(file_path: &str) -> isize {
    0
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
        assert_eq!(maze2("src/d10/input.txt"), 1);
    }
}
