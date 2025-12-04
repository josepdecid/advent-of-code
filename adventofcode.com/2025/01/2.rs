fn read_data() -> Vec<(isize, isize)>  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);
    
    return data
        .lines()
        .map(|line| {
            let parts = line.split_at(1);
			let direction = if parts.0.chars().next().unwrap() == 'R' { 1 } else { -1 };
			let steps = parts.1.parse::<isize>().unwrap();
            return (direction, steps);
        })
        .collect()
}

fn main() {
	let mut zeros_count = 0;
	let mut position = 50;
	for (direction, steps) in read_data() {
		zeros_count += steps / 100;
		let remaining_steps = steps % 100;
		
		let zero_distance = (100 - direction * position) % 100;
		if 0 < zero_distance && zero_distance <= remaining_steps {
			zeros_count += 1;
		}
		
		position = (position + direction * remaining_steps) % 100;
	}
    
	println!("Result: {}", zeros_count); // 6175
}
