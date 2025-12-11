fn read_data() -> Vec<Vec<char>>  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

	return data
		.lines()
		.map(|line| line.chars().collect())
		.collect();
}

fn fire_beam(map: &mut Vec<Vec<char>>) -> usize {
	let initial_beam_position = map[0].iter().position(|&c| c == 'S').unwrap();
	
	let mut count_beams: Vec<usize> = vec![0; map[0].len()];
	count_beams[initial_beam_position] = 1;

	for i in 1..map.len() {
		for j in 0..map[i].len() {
			if map[i][j] == '^' && count_beams[j] > 0 {
				count_beams[j - 1] += count_beams[j];
				count_beams[j + 1] += count_beams[j];
				count_beams[j] = 0;
			}
		}
	}

	return count_beams.iter().sum();
}

fn main() {
    let mut map = read_data();
	let count = fire_beam(&mut map);
	println!("Result: {}", count); // 5748679033029
}
