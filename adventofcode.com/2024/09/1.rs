fn read_data() -> Vec<i64>  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);
    
    return data
        .trim()
        .chars()
        .map(|v| v.to_digit(10).unwrap() as i64).collect();
}

fn get_data_representation(data: &Vec<i64>) -> Vec<i64> {
    let mut i: usize = 0;
    let mut representation: Vec<i64> = vec![-1; data.iter().sum::<i64>() as usize];

    let mut current_id: i64 = 0;
    let mut is_block = true;

    for value in data {
        for _ in 0..*value {
            representation[i] = if is_block { current_id } else { -1 };
            i += 1;
        }

        if is_block {
            is_block = false;
        } else {
            is_block = true;
            current_id += 1;
        }
    }

    return representation;
}

fn compact_disk(representation: &mut Vec<i64>) {
    let mut start_idx = 0;
    let mut end_idx = representation.len() - 1;

    while start_idx < end_idx {
        if representation[start_idx] == -1 {
            representation[start_idx] = representation[end_idx];
            representation[end_idx] = -1;
            
            while representation[end_idx] == -1 {
                end_idx -= 1;
            }
        }

        start_idx += 1;
    }
}

fn calculate_checksum(representation: &Vec<i64>) -> i64 {
    let mut result: i64 = 0;
    let mut i: usize = 0;

    while i < representation.len() {
        if representation[i] == -1 {
            break;
        }

        result += representation[i] * i as i64;
        i += 1;
    }

    return result;
}

fn main() {
    let data = read_data();
    let mut data_representation = get_data_representation(&data);
    compact_disk(&mut data_representation);

    let result = calculate_checksum(&data_representation);
    println!("Result: {}", result);  // Result: 6331212425418 
}
