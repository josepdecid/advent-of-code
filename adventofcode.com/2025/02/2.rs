fn read_data() -> Vec<(usize, usize)>  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);
    
    return data
        .split(',')
        .map(|pair| {
            let mut nums = pair.split('-').map(|val| val.parse::<usize>().unwrap());
            let start = nums.next().unwrap();
            let end = nums.next().unwrap();
            (start, end)
        })
        .collect();
}


fn does_repeat_pattern_across_value(value: &String, pattern: String) -> bool {
    let digits = value.len();
    let step = pattern.len();

    for i in (step..=(digits - step)).step_by(step) {
        if value[i..(i + step)] != pattern {
            return false;
		}
	}

    return true;
}


fn is_invalid_id(value: &String) -> bool {
	let digits = value.len();
	let half_digits = digits / 2;
	for i in 1..=half_digits {
		let pattern = value[..i].to_string();
		if digits % i == 0 && does_repeat_pattern_across_value(&value, pattern) {
			return true;
		}
	}

	return false;
}


fn main() {
	let mut invalid_id_sum = 0;
	for (start, end) in read_data() {
		for i in start..end {
			if is_invalid_id(&i.to_string()) {
				invalid_id_sum += i;
			}
		}
	}
    
	println!("Result: {}", invalid_id_sum); // 48631958998
}
