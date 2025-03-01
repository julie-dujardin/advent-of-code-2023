use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn parse_lines(file_path: &str) -> HashMap<String, HashSet<String>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut connections = HashMap::new();
    for line in file.lines() {
        let (source_raw, destinations_raw) = line.split_once(": ").unwrap();
        let destinations = destinations_raw
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let source = source_raw.to_string();

        connections
            .entry(source.clone())
            .or_insert_with(HashSet::new);

        for destination in destinations {
            connections
                .entry(destination.clone())
                .or_insert_with(HashSet::new)
                .insert(source.clone());
            connections.get_mut(&source).unwrap().insert(destination);
        }
    }
    connections
}

fn find_shortest_path(
    start: &String,
    end: &String,
    connections: &HashMap<String, HashSet<String>>,
    path_cache: &mut HashMap<(String, String), Option<Vec<(String, String)>>>,
) -> Option<Vec<(String, String)>> {
    // Check if this path is already in cache
    let cache_key = if start < end {
        (start.clone(), end.clone())
    } else {
        (end.clone(), start.clone())
    };

    if let Some(cached_result) = path_cache.get(&cache_key) {
        // If the path is in reverse order, we need to reverse the edges
        if start > end {
            return cached_result.as_ref().map(|p| p.clone());
        } else {
            return cached_result.as_ref().map(|p| p.clone());
        }
    }

    // BFS implementation
    let mut queue = VecDeque::new();
    queue.push_back((start.clone(), Vec::new()));

    let mut visited = HashSet::new();
    visited.insert(start.clone());

    // Keep track of parent nodes to reconstruct path
    let mut parent: HashMap<String, (String, (String, String))> = HashMap::new();

    let result = loop {
        if queue.is_empty() {
            break None;
        }

        let (current, path) = queue.pop_front()?;

        if &current == end {
            break Some(path);
        }

        if let Some(neighbors) = connections.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());

                    // Create normalized edge (smaller node name first)
                    let edge = if current < *neighbor {
                        (current.clone(), neighbor.clone())
                    } else {
                        (neighbor.clone(), current.clone())
                    };

                    let mut new_path = path.clone();
                    new_path.push(edge.clone());

                    parent.insert(neighbor.clone(), (current.clone(), edge));
                    queue.push_back((neighbor.clone(), new_path));
                }
            }
        }
    };

    // Store the result in cache
    path_cache.insert(cache_key, result.clone());

    result
}

fn count_nodes(start: &String, graph: &HashMap<String, HashSet<String>>) -> usize {
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut destinations = graph.get(start).unwrap().iter().collect::<Vec<&String>>();

    while destinations.len() > 0 {
        let curr_dest = destinations.pop().unwrap();
        if !visited.contains(curr_dest) {
            destinations.extend(graph.get(curr_dest).unwrap().iter())
        }
        visited.insert(curr_dest);
    }

    visited.len()
}

pub fn snow1(file_path: &str) -> usize {
    let mut connections = parse_lines(file_path);
    let nodes = connections.keys().collect::<Vec<&String>>();
    let mut pass_count = HashMap::new();
    let mut path_cache = HashMap::new();

    for _ in 0..2000 {
        let source = nodes.choose(&mut thread_rng()).unwrap();
        let destination = nodes.choose(&mut thread_rng()).unwrap();

        if let Some(path) = find_shortest_path(source, destination, &connections, &mut path_cache) {
            for edge in path.into_iter() {
                pass_count.entry(edge).and_modify(|x| *x += 1).or_insert(1);
            }
        }
    }

    let mut pass_count_sorted = pass_count
        .into_iter()
        .map(|(node, count)| (node, count))
        .collect::<Vec<((String, String), usize)>>();
    pass_count_sorted.sort_by(|(_, x1), (_, x2)| x2.cmp(x1));

    let (g1, g2) = &pass_count_sorted[0].0;

    for ((source, destination), _) in pass_count_sorted[..3].iter() {
        connections
            .entry(source.clone())
            .and_modify(|destinations| {
                destinations.remove(destination);
            });
        connections
            .entry(destination.clone())
            .and_modify(|destinations| {
                destinations.remove(source);
            });
    }

    count_nodes(g1, &connections) * count_nodes(g2, &connections)
}

pub fn snow2(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d25", "p1", snow1);
    }

    // #[test]
    // fn p2() {
    //     check_results("d25", "p2", snow2);
    // }
}
