// https://adventofcode.com/2024/day/5
use std::collections::{HashMap,HashSet};

fn fix_order_and_ged_middle_value(mut values: Vec<u32>, value_to_dependencies: &HashMap<u32, Vec<u32>>) -> u32 {
    loop {
        let mut swapped = false; 

        for i in 0..(values.len() - 1) {
            if let Some(dependencies) = value_to_dependencies.get(&values[i]) {
                if dependencies.contains(&values[i + 1]) {
                    values.swap(i, i + 1);
                    swapped = true;
                }
            }
        }

        if !swapped {
            return values[values.len() / 2];
        }
    }
}

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
                    return Some(fix_order_and_get_middle_value(values, value_to_dependencies));
                }
            }
        }

        prev_values.insert(*value);
    }

    return None;
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

    println!("Result: {}", result);  // 4077
}

