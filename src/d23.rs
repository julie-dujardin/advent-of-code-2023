use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_map(file_path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut lines = Vec::new();
    for line in file.lines() {
        lines.push(line.chars().collect());
    }
    lines
}

fn insert_node_to_graph(
    last_node: (i32, i32),
    new_node: (i32, i32),
    distance: usize,
    graph: &mut HashMap<(i32, i32), HashMap<(i32, i32), usize>>,
    ignore_slopes: bool,
) {
    graph.entry(last_node).and_modify(|x| {
        x.insert(new_node, distance);
    });
    if ignore_slopes {
        graph
            .entry(new_node)
            .and_modify(|x| {
                x.insert(last_node, distance);
            })
            .or_insert_with(|| {
                let mut new_map = HashMap::new();
                new_map.insert(last_node, distance);
                new_map
            });
    } else {
        graph.entry(new_node).or_insert_with(HashMap::new);
    }
}

/// Depth-first search: go to the next node, then explore the next unexplored branch.
fn build_graph(
    map: &Vec<Vec<char>>,
    ignore_slopes: bool,
) -> HashMap<(i32, i32), HashMap<(i32, i32), usize>> {
    let mut graph = HashMap::new();
    let mut visited = HashSet::new();
    let first_node = (1, 0);
    let final_node = ((map[0].len() - 2) as i32, (map.len() - 1) as i32);
    let mut next_positions = vec![(first_node, first_node, 0)];
    graph.insert(first_node, HashMap::new());

    while next_positions.len() > 0 {
        let ((start_x, start_y), last_node, distance_from_node) = next_positions.pop().unwrap();

        if start_x == final_node.0 && start_y == final_node.1 {
            insert_node_to_graph(
                last_node,
                final_node,
                distance_from_node,
                &mut graph,
                ignore_slopes,
            );
        }

        let mut new_positions = Vec::new();
        let mut neighbor_count = 0;
        for (check_x_offset, check_y_offset) in vec![(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let check_x = start_x + check_x_offset;
            let check_y = start_y + check_y_offset;
            // out-of-bounds
            if check_x < 0
                || check_y < 0
                || check_x >= map[0].len() as i32
                || check_y >= map.len() as i32
            {
                continue;
            }
            // can't be a wall or already visited
            let check_tile = map[check_y as usize][check_x as usize];
            if check_tile == '#' {
                continue;
            }
            neighbor_count += 1;
            // go the correct way
            if check_tile == '^' && check_y_offset != -1
                || check_tile == 'v' && check_y_offset != 1
                || check_tile == '>' && check_x_offset != 1
                || check_tile == '<' && check_x_offset != -1
            {
                if !ignore_slopes {
                    continue;
                }
            }

            new_positions.push(((check_x, check_y), last_node, distance_from_node + 1));
        }
        // If, from the current starting point, we reached 2 valid neighbors (including those already traversed), we're on a node.
        if neighbor_count > 2 {
            let new_node = (start_x, start_y);
            if new_node != last_node {
                insert_node_to_graph(
                    last_node,
                    new_node,
                    distance_from_node,
                    &mut graph,
                    ignore_slopes,
                );
                new_positions = new_positions
                    .iter()
                    .map(|((check_x, check_y), _last_node, _distance_from_node)| {
                        ((*check_x, *check_y), (start_x, start_y), 1) // start at 1 to account for the cost of leaving the node
                    })
                    .collect();
            }
        }

        // Leave the visited check here, so we can reach previously visited nodes again.
        // We just have to not start new branches from those nodes.
        if !visited.contains(&(start_x, start_y)) {
            visited.insert((start_x, start_y));
            next_positions.extend(new_positions);
        }
    }

    graph
}

pub fn print_graph(map: &Vec<Vec<char>>, graph: &HashMap<(i32, i32), HashMap<(i32, i32), usize>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if graph.contains_key(&(x as i32, y as i32)) {
                print!("N");
            } else {
                print!("{}", map[y][x]);
            }
        }
        println!()
    }

    println!("{:?}", graph);
}

fn get_max_graph_len(
    graph: &HashMap<(i32, i32), HashMap<(i32, i32), usize>>,
    final_node: (i32, i32),
    curr_node: (i32, i32),
    visited: HashSet<(i32, i32)>,
    path_length: usize,
) -> usize {
    if visited.contains(&curr_node) {
        return 0;
    }
    let mut new_visited = visited.clone();
    new_visited.insert(curr_node);

    if curr_node == final_node {
        return path_length;
    }

    let distances = graph
        .get(&curr_node)
        .unwrap()
        .iter()
        .map(|(next_node, distance)| {
            get_max_graph_len(
                &graph,
                final_node,
                *next_node,
                new_visited.clone(),
                path_length + distance,
            )
        });

    distances.max().unwrap()
}

pub fn walk1(file_path: &str) -> usize {
    let map = parse_map(file_path);
    let graph = build_graph(&map, false);

    print_graph(&map, &graph);

    let final_node = ((map[0].len() - 2) as i32, (map.len() - 1) as i32);
    get_max_graph_len(&graph, final_node, (1, 0), HashSet::new(), 0)
}

pub fn walk2(file_path: &str) -> usize {
    let map = parse_map(file_path);
    let graph = build_graph(&map, true);

    print_graph(&map, &graph);

    let final_node = ((map[0].len() - 2) as i32, (map.len() - 1) as i32);
    get_max_graph_len(&graph, final_node, (1, 0), HashSet::new(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d23", "p1", walk1);
    }

    #[test]
    fn p2() {
        check_results("d23", "p2", walk2);
    }
}
