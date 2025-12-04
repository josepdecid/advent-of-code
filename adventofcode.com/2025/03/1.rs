use std::cmp::max;

fn read_data() -> Vec<Vec<usize>>  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);
    
    return data
        .lines()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>()
        )
        .collect()
}

fn calculate_highest_bank_value(bank: &Vec<usize>) -> usize {
	let mut highest_value = 0;
	for i in 0..bank.len() {
        for j in (i + 1)..bank.len() {
            highest_value = max(highest_value, bank[i] * 10 + bank[j]);
		}
	}

    return highest_value;
}

fn main() {
	let mut battery_sum = 0;
	for bank in read_data() {
		battery_sum += calculate_highest_bank_value(&bank);
	}
    
	println!("Result: {}", battery_sum); // 17346
}
