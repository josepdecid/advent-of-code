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


fn is_invalid_id(value: String) -> bool {
	let digits = value.len();
	if digits % 2 != 0 {
		return false;
	}

	let half_digits = digits / 2;
	let (first_part, second_part) = value.split_at(half_digits);
	if first_part != second_part {
		return false;
	}

	return true;
}


fn main() {
	let mut invalid_id_sum = 0;
	for (start, end) in read_data() {
		for i in start..end {
			if is_invalid_id(i.to_string()) {
				invalid_id_sum += i;
			}
		}
	}
    
	println!("Result: {}", invalid_id_sum); // 29940924880
}
