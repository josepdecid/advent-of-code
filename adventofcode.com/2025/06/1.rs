fn read_data() -> (Vec<Vec<usize>>, Vec<char>)  {
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

    let mut values: Vec<Vec<usize>> = vec![Vec::new(); num_operations];
    for line in operations_lines {
        let terms: Vec<&str> = line.split_whitespace().collect();
        terms.iter().enumerate().for_each(|(i, value)| {
            values[i].push(value.parse::<usize>().unwrap());
        });
    }
    
    return (values, symbols);
}

fn main() {
    let (values, symbols) = read_data();
    let sum_of_results: usize = values.iter()
        .zip(&symbols)
        .map(|(vals, &op)| match op {
            '+' => vals.iter().sum::<usize>(),
            '*' => vals.iter().product::<usize>(),
            op => panic!("Unsupported operation: {}", op),
        })
        .sum();
    
    println!("Result: {}", sum_of_results); // 3968933219902
}
