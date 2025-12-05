use std::cmp::max;

fn read_data() -> Vec<(usize, usize)>  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for line in data.lines() {
		if line == "" {
			break;
		}

		let range: Vec<_> = line.split("-").collect();
		ranges.push((
			range[0].parse::<usize>().unwrap(),
			range[1].parse::<usize>().unwrap()
		));
    }
    
	return ranges;
}

fn main() {
    let mut ranges = read_data();
	ranges.sort_by_key(|range| range.0);
	
	let mut fresh_items_count = 0;
	let (mut visited_start, mut visited_end) = ranges[0];

	for (start, end) in ranges.into_iter().skip(1) {
		if start <= visited_end {
			visited_end = max(visited_end, end);
			continue;
		}
		
		fresh_items_count += visited_end - visited_start + 1;
		visited_start = start;
		visited_end = end;
	}

	fresh_items_count += visited_end - visited_start + 1;

	println!("Result: {}", fresh_items_count); // 344771884978261
}
