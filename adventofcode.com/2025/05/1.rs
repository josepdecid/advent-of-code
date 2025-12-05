fn read_data() -> (Vec<(usize, usize)>, Vec<usize>)  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

    let mut ranges: Vec<(usize, usize)> = Vec::new();
	let mut items: Vec<usize> = Vec::new();
	let mut has_read_ranges = false;

    for line in data.lines() {
		if line == "" {
			has_read_ranges = true;
			continue;
		}

		if !has_read_ranges {
			let range: Vec<_> = line.split("-").collect();
			ranges.push((
				range[0].parse::<usize>().unwrap(),
				range[1].parse::<usize>().unwrap()
			));
		} else {
			items.push(line.parse::<usize>().unwrap());
		}
    }
    
	return (ranges, items);
}

fn main() {
    let (ranges, items) = read_data();
	let mut fresh_items_count = 0;

	for item in items {
		for (start, end) in ranges.clone() {
			if item >= start && item <= end {
				fresh_items_count += 1;
				break;
			}
		}
	}
    
	println!("Result: {}", fresh_items_count); // 821
}
