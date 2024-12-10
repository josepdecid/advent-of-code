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
    let mut file_start_idx = representation.len() - 1;
    let mut file_end_idx = representation.len() - 1;
    let mut file_id = representation[file_start_idx];

    let mut first_empty_idx = 0;
    
    while file_id > 0 {
        // Find end index of current file ID
        for i in (0..(file_end_idx + 1)).rev() {
            if representation[i] == file_id {
                file_end_idx = i;
                break;
            }
        }
        
        // Find sart index of current file ID
        for i in (0..(file_end_idx + 1)).rev() {
            if representation[i] != file_id {
                file_start_idx = i + 1;
                break;
            }
        } 

        let file_size = 1 +  file_end_idx - file_start_idx;

        // Move to first empty index so we don't need to search from 0 every time
        while representation[first_empty_idx] != -1 {
            first_empty_idx += 1;
        }
    
        // Find first empty index that can fit the file
        let mut empty_idx = first_empty_idx;
        let mut empty_count = 0;
        for i in first_empty_idx..file_start_idx {
            if representation[i] != -1 {
                empty_count = 0;
                continue;
            }

            if empty_count == 0 {
                empty_idx = i;
            }

            empty_count += 1;
            if empty_count == file_size {
                break;
            }
        }
        
        // If the file fits somewhere, move it to the empty space
        if empty_count == file_size {
            for i in 0..file_size {
                representation[empty_idx + i] = file_id;
                representation[file_start_idx + i] = -1;
            }
        }

        // Move to previous ID as those are assigned incrementally
        file_id -= 1;
    } 
}

fn calculate_checksum(representation: &Vec<i64>) -> i64 {
    let mut result: i64 = 0;
    let mut i: usize = 0;

    while i < representation.len() {
        if representation[i] != -1 {
            result += representation[i] * i as i64;
        }

        i += 1;
    }

    return result;
}

fn main() {
    let data = read_data();
    let mut data_representation = get_data_representation(&data);
    compact_disk(&mut data_representation);

    let result = calculate_checksum(&data_representation);
    println!("Result: {}", result);  // Result: 6363268339304
}

