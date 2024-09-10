// Required imports
use std::fs;
use toml::Value;

pub fn load_results(day_dir: &str) -> Result<(std::collections::HashMap<String, i64>, std::collections::HashMap<String, i64>), Box<dyn std::error::Error>> {
    // Read the TOML file for the given day
    let toml_path = format!("test-data/{}/results.toml", day_dir);
    let content = fs::read_to_string(toml_path)?;

    // Parse the TOML content
    let parsed: Value = toml::from_str(&content)?;

    // Convert part1 and part2 into HashMaps with filenames as keys and expected values as i64
    let part1 = parsed["p1"].as_table().unwrap()
        .iter()
        .map(|(key, val)| (key.clone(), val.as_integer().unwrap()))
        .collect();

    let part2 = parsed["p2"].as_table().unwrap()
        .iter()
        .map(|(key, val)| (key.clone(), val.as_integer().unwrap()))
        .collect();

    Ok((part1, part2))
}