fn read_data() -> Vec<(i64, i64)> {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

	return data
		.lines()
		.map(|line| {
			let coords = line
				.split(",")
				.map(|v| v.parse().unwrap())
				.collect::<Vec<i64>>();
			return (coords[0], coords[1]);
		})
		.collect();
}

fn find_largest_rectangle(tiles: &Vec<(i64, i64)>) -> u64 {
	let mut max_area = 0;
	for i in 0..tiles.len() {
		for j in (i + 1)..tiles.len() {
			let (x1, y1) = tiles[i];
			let (x2, y2) = tiles[j];
			let area = ((x1 - x2 + 1).abs() * (y1 - y2 + 1).abs()) as u64;
			if area > max_area {
				max_area = area;
			}
		}
	}

	return max_area;
}

fn main() {
    let tiles = read_data();
	let area = find_largest_rectangle(&tiles);
	println!("Result: {}", area); // 4763040296
}
