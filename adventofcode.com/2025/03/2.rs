use std::cmp::{max,min};
use std::collections::HashMap;

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
	let n = bank.len();
	if n < 12 {
		return 0;
	}

	// Use Dynamic Programming lookup table:
	// dp[i, k] = maximum value using k digits from positions [i, n)
	// Base case with k=0 where the value is 0
	let mut dp: HashMap<(usize, usize), usize> = HashMap::new();
	for i in 0..=n {
		dp.insert((i, 0), 0);
	}

	for i in (0..n).rev() {
		for k in 1..=min(12, n - i) {
			// Option 1: Include bank[i] as the (12-k+1)-th digit
			let include_val = dp
				.get(&(i + 1, k - 1))
				.map(|&val| bank[i] * 10usize.pow((k - 1) as u32) + val);

			// Option 2: Skip bank[i]
			let skip_val = dp.get(&(i + 1, k)).copied();

			let maximum_value = match (include_val, skip_val) {
				(Some(a), Some(b)) => max(a, b),
				(Some(a), None) => a,
				(None, Some(b)) => b,
				(None, None) => 0,
			};

			dp.insert((i, k), maximum_value);
		}
	}

    return *dp.get(&(0, 12)).unwrap_or(&0);
}

fn main() {
	let mut battery_sum = 0;
	for bank in read_data() {
		battery_sum += calculate_highest_bank_value(&bank);
	}
    
	println!("Result: {}", battery_sum); // 172981362045136
}
