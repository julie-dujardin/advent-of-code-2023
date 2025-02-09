use std::collections::HashMap;
use std::fs;

fn parse_file(file_path: &str) -> (HashMap<String, Vec<String>>, Vec<HashMap<char, usize>>) {
    let file = fs::read_to_string(file_path).unwrap();
    let components = file.split("\n\n").collect::<Vec<&str>>();

    let mut workflows = HashMap::new();
    for raw_workflow in components[0].lines() {
        let workflow_components = raw_workflow
            .split(|c| c == '{' || c == '}')
            .collect::<Vec<&str>>();
        workflows.insert(
            workflow_components[0].to_string(),
            workflow_components[1]
                .split(',')
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
        );
    }

    let mut parts = Vec::new();
    for raw_part in components[1].lines() {
        let mut curr_part = HashMap::new();
        for raw_attribute in raw_part
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
        {
            let attribute_components = raw_attribute.split('=').collect::<Vec<&str>>();
            curr_part.insert(
                attribute_components[0].chars().nth(0).unwrap(),
                attribute_components[1].parse::<usize>().unwrap(),
            );
        }
        parts.push(curr_part);
    }

    (workflows, parts)
}

pub fn plenty1(file_path: &str) -> usize {
    let (workflows, parts) = parse_file(file_path);

    let mut accepted = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        let mut curr_workflow_name = "in".to_string();
        // for (key, value) in part {
        //     print!("{}={}", key, value);
        // }
        // print!("\t");

        while curr_workflow_name != "A" && curr_workflow_name != "R" {
            // print!("{curr_workflow_name} -> ");
            for operation in &workflows[&curr_workflow_name] {
                if operation.contains('<') || operation.contains('>') {
                    let operation_components = operation
                        .split(|c| c == '>' || c == '<' || c == ':')
                        .collect::<Vec<&str>>();
                    let operation_property = operation_components[0].chars().nth(0).unwrap();
                    let operation_threshold = operation_components[1].parse::<usize>().unwrap();
                    let operation_destination = operation_components[2].to_string();

                    if operation.contains('<') {
                        if part[&operation_property] < operation_threshold {
                            curr_workflow_name = operation_destination;
                            break;
                        }
                    } else {
                        if part[&operation_property] > operation_threshold {
                            curr_workflow_name = operation_destination;
                            break;
                        }
                    }
                } else {
                    curr_workflow_name = operation.clone()
                }
            }
        }
        // println!("{curr_workflow_name}");
        if curr_workflow_name == "A" {
            accepted.push(i)
        }
    }

    let mut result = 0;
    for i in accepted {
        result += parts[i].values().sum::<usize>();
    }
    result
}

fn split_interval(interval: &(usize, usize), pivot: usize) -> ((usize, usize), (usize, usize)) {
    ((interval.0, pivot - 1), (pivot, interval.1))
}

pub fn plenty2(file_path: &str) -> usize {
    let (workflows, _) = parse_file(file_path);

    let intervals = HashMap::from([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]);

    let mut workflow_intervals = vec![("in".to_string(), intervals)];
    let mut accepted_intervals = Vec::new();

    while !workflow_intervals.is_empty() {
        let (curr_workflow_name, mut curr_intervals) = workflow_intervals.pop().unwrap();

        if curr_workflow_name == "A" || curr_workflow_name == "R" {
            if curr_workflow_name == "A" {
                accepted_intervals.push(curr_intervals);
            }
            continue;
        }

        for operation in &workflows[&curr_workflow_name] {
            if operation.contains('<') || operation.contains('>') {
                let operation_components = operation
                    .split(|c| c == '>' || c == '<' || c == ':')
                    .collect::<Vec<&str>>();
                let operation_property = operation_components[0].chars().nth(0).unwrap();
                let operation_threshold = operation_components[1].parse::<usize>().unwrap();
                let operation_destination = operation_components[2].to_string();

                let mut new_intervals = curr_intervals.clone();

                if operation.contains('<') {
                    if curr_intervals[&operation_property].0 < operation_threshold {
                        let split_intervals = split_interval(
                            curr_intervals.get(&operation_property).unwrap(),
                            operation_threshold,
                        );

                        new_intervals.insert(operation_property, split_intervals.0);
                        curr_intervals.insert(operation_property, split_intervals.1);
                        workflow_intervals.push((operation_destination, new_intervals));
                    }
                } else {
                    if curr_intervals[&operation_property].1 > operation_threshold {
                        let split_intervals = split_interval(
                            curr_intervals.get(&operation_property).unwrap(),
                            operation_threshold + 1,
                        );

                        new_intervals.insert(operation_property, split_intervals.1);
                        curr_intervals.insert(operation_property, split_intervals.0);
                        workflow_intervals.push((operation_destination, new_intervals));
                    }
                }
            } else {
                workflow_intervals.push((operation.clone(), curr_intervals.clone()));
            }
        }
    }

    let mut combinations = 0;
    for accepted in accepted_intervals {
        // for (key, value) in &accepted {
        //     print!("{}={},{};\t", key, value.0, value.1);
        // }
        // println!();
        combinations += i_size(accepted[&'x'])
            * i_size(accepted[&'m'])
            * i_size(accepted[&'a'])
            * i_size(accepted[&'s']);
    }
    combinations
}

fn i_size(interval: (usize, usize)) -> usize {
    interval.1 - interval.0 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d19", "p1", plenty1);
    }

    #[test]
    fn p2() {
        check_results("d19", "p2", plenty2);
    }
}
