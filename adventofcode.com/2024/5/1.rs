// https://adventofcode.com/2024/day/5
use std::collections::{HashMap,HashSet};

fn get_middle_value_if_correct(line: &str, value_to_dependencies: &HashMap<u32, Vec<u32>>) -> Option<u32> {
    let values: Vec<u32> = line
        .split(",")
        .map(|val| val.parse::<u32>().unwrap())
        .collect(); 

    let values_set: HashSet<u32> = values.iter().cloned().collect();
    let mut prev_values: HashSet<u32> = HashSet::new();
    
    for value in &values {
        if let Some(dependencies) = value_to_dependencies.get(value) {
            for dependency in dependencies {
                if !prev_values.contains(dependency) && values_set.contains(dependency) {
                    return None;
                }
            }
        }

        prev_values.insert(*value);
    }

    return Some(values[values.len() / 2]);
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

    let mut result = 0; 

    let mut dependencies_mode = true;
    let mut value_to_dependencies: HashMap<u32, Vec<u32>> = HashMap::new();

    let lines: Vec<&str> = data.lines().collect();
    for line in lines {
        if line == "" {
            dependencies_mode = false;
            continue;
        }

        if dependencies_mode {
            let parts: Vec<u32> = line
                .split("|")
                .map(|val| val.parse::<u32>().unwrap())
                .collect();
            value_to_dependencies
                .entry(parts[1])
                .or_insert_with(Vec::new)
                .push(parts[0]);
            continue;
        }
        
        result += match get_middle_value_if_correct(line, &value_to_dependencies) {
            Some(p) => p,
            None => 0
        };
    }

    println!("Result: {}", result);  // 5129
}
