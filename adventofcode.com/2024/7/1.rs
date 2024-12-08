fn read_data() -> Vec<(u64, Vec<u64>)>  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);
    
    return data
        .lines()
        .map(|line| {
            let (result, terms) = line.split_once(':').unwrap();
            let result = result.parse().unwrap();
            (
                result,
                terms.split_whitespace().map(|n| n.parse().unwrap()).collect()
            )
        })
        .collect();
}

fn is_equation_solvable(result: &u64, terms: &Vec<u64>) -> bool {
    let number_of_operations = (terms.len() - 1) as u32;
    let total_combinations = 2_u32.pow(number_of_operations);

    for i in 0..total_combinations {
        let mut combination_result = terms[0];
        let combination = (0..number_of_operations)
            .map(|j| ((i / 2_u32.pow(j)) % 2) as u8)
            .collect::<Vec<u8>>();
        
        for j in 1..terms.len() {
            if combination[j - 1] == 0 {
                combination_result += terms[j];
            } else {
                combination_result *= terms[j];
            }
        }

        if combination_result == *result {
            return true;
        }
    }

    return false;
}

fn main() {
    let data = read_data();
    
    let mut count = 0;
    for equation in data {
        let result = equation.0;
        let terms = equation.1;

        if is_equation_solvable(&result, &terms) {
            count += result;
        }
    }

    println!("Result: {}", count);  // 2437272016585  
}
