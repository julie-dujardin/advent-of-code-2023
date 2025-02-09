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

        while curr_workflow_name != "A" && curr_workflow_name != "R"  {
            // print!("{curr_workflow_name} -> ");
            for operation in &workflows[&curr_workflow_name] {
                if operation.contains('<') || operation.contains('>') {
                    let operation_components = operation.split(|c| c == '>' || c == '<' || c == ':').collect::<Vec<&str>>();
                    let operation_property = operation_components[0].chars().nth(0).unwrap();
                    let operation_threshold = operation_components[1].parse::<usize>().unwrap();
                    let operation_destination = operation_components[2].to_string();

                    if operation.contains('<') {
                        if part[&operation_property] < operation_threshold {
                            curr_workflow_name = operation_destination;
                            break;
                        }
                    }
                    else {
                        if part[&operation_property] > operation_threshold {
                            curr_workflow_name = operation_destination;
                            break;
                        }
                    }
                }
                else {
                    curr_workflow_name = operation.clone()
                }
            }
        }
        // println!("{curr_workflow_name}");
        if curr_workflow_name == "A" {accepted.push(i)}
    }

    let mut result = 0;
    for i in accepted{
        result += parts[i].values().sum::<usize>();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d19", "p1", plenty1);
    }

    // #[test]
    // fn p2() {
    //     check_results("d19", "p2", plenty2);
    // }
}
