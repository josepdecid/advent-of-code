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
		position = (position + direction * steps) % 100;
		if position == 0 {
			zeros_count += 1;
		}
	}
    
	println!("Result: {}", zeros_count); // 1102
}
