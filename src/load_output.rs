// Required imports
use std::fs;
use toml::Value;

pub fn load_results(day_dir: &str, part: &str) -> std::collections::HashMap<String, usize> {
    // Read the TOML file for the given day
    let toml_path = format!("test-data/{}/results.toml", day_dir);
    let content = fs::read_to_string(toml_path).unwrap();

    // Parse the TOML content
    let parsed: Value = toml::from_str(&content).unwrap();

    // Convert part1 and part2 into HashMaps with filenames as keys and expected values as i64
    parsed[part]
        .as_table()
        .unwrap()
        .iter()
        .map(|(key, val)| (key.clone(), val.as_integer().unwrap() as usize))
        .collect()
}
