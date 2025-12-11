use std::cmp::max;

fn calculate_max_digits_per_operation(lines: &Vec<&str>, num_operations: usize) -> Vec<usize> {
    let mut max_digits: Vec<usize> = vec![0; num_operations];
    for line in lines.clone() {
        for (i, value) in line.split_whitespace().enumerate() {
            max_digits[i] = max(max_digits[i], value.len());
        }
    }

    return max_digits;
}

fn extract_raw_values_per_operation<'a>(
    lines: &'a Vec<&str>,
    num_operations: usize,
    max_digits: &Vec<usize>,
) -> Vec<Vec<&'a str>> {
    let mut start_idx: usize = 0;
    let mut raw_values = vec![vec![""; lines.len()]; num_operations];

    for i in 0..num_operations {
        let end_idx = start_idx + max_digits[i];
        for (line_idx, line) in lines.clone().into_iter().enumerate() {
            raw_values[i][line_idx] = &line[start_idx..end_idx];
        }

        start_idx = end_idx + 1;
    }

    return raw_values;
}

fn transpose_and_parse_operands(
    values: &Vec<Vec<&str>>,
    num_operations: usize,
    max_digits: &Vec<usize>,
) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    for i in 0..num_operations {
        result.push(vec![0; max_digits[i]]);
        for j in 0..max_digits[i] {
            for value in values[i].iter() {
                let char_value = value.chars().nth(j).unwrap();
                if char_value == ' ' {
                    continue;
                }
                
                result[i][j] *= 10;
                result[i][j] += char_value.to_digit(10).unwrap() as usize;
            }
        }
    }

    return result;
}

fn read_data() -> (Vec<Vec<usize>>, Vec<char>) {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

    let mut operations_lines: Vec<&str> = data.lines().collect();
    
    let symbols_line = operations_lines.pop();
    let symbols: Vec<char> = symbols_line
        .unwrap()
        .split_whitespace()
        .map(|symbol| symbol.chars().next().unwrap())
        .collect();
    
    let num_operations = symbols.len();

    let max_digits = calculate_max_digits_per_operation(&operations_lines, num_operations);
    let raw_values = extract_raw_values_per_operation(&operations_lines, num_operations, &max_digits);
    let values = transpose_and_parse_operands(&raw_values, num_operations, &max_digits);

    return (values, symbols);
}

fn main() {
    let (values, symbols) = read_data();
    let sum_of_results: usize = values
        .iter()
        .zip(&symbols)
        .map(|(vals, &op)| match op {
            '+' => vals.iter().sum::<usize>(),
            '*' => vals.iter().product::<usize>(),
            op => panic!("Unsupported operation: {}", op),
        })
        .sum();

    println!("Result: {}", sum_of_results); // 6019576291014
}
