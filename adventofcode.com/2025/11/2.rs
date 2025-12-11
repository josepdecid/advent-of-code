use std::collections::HashMap;

const SOURCE: &str = "svr";
const TARGET: &str = "out";

fn read_data() -> HashMap<String, Vec<String>> {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

	let mut map: HashMap<String, Vec<String>> = HashMap::new();
	for line in data.lines() {
		let parts: Vec<&str> = line.split(" ").collect();
		
		let source = &parts[0][..(parts[0].len() - 1)].to_string();
		let targets = parts[1..parts.len()]
			.iter()
			.map(|s| s.to_string())
			.collect::<Vec<String>>();
		
		map.insert(source.clone(), targets);
	}

	return map;
}

fn count_paths_to_out(
	source: String,
	data: &HashMap<String, Vec<String>>,
	partial_counts: &mut HashMap<String, usize>,
	visited_dac: bool,
	visited_fft: bool,
) -> usize {
	if source == TARGET {
		return (visited_dac && visited_fft) as usize;
	}

	let new_visited_dac = visited_dac || source == "dac";
	let new_visited_fft = visited_fft || source == "fft";

	let mut count = 0;
	for output in data.get(&source).unwrap() {		
		let partial_key = format!("{}_{}_{}", output, new_visited_dac, new_visited_fft);
		if partial_counts.contains_key(&partial_key) {
			count += partial_counts.get(&partial_key).unwrap();
			continue;
		}

		let sub_count = count_paths_to_out(
			output.clone(),
			data,
			partial_counts,
			new_visited_dac,
			new_visited_fft,
		);

		partial_counts.insert(partial_key, sub_count);
		count += sub_count;
	}

	return count;
}

fn main() {
    let data = read_data();
	
	let mut partial_counts: HashMap<String, usize> = HashMap::new();
	let result = count_paths_to_out(
		String::from(SOURCE),
		&data,
		&mut partial_counts,
		false,
		false,
	);

	println!("Result: {}", result); // 499645520864100
}
