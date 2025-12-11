use std::collections::VecDeque;

fn read_data() -> Vec<(u64, Vec<u64>)> {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

	return data
		.lines()
		.map(|line| {
			let parts: Vec<&str> = line.split(" ").collect();
			
			let mut lights = 0u64;
			let raw_lights = &parts[0][1..(parts[0].len() - 1)];
			for (i, ch) in raw_lights.chars().enumerate() {
				if ch == '#' {
					lights |= 1u64 << i;
				}
			}
			
			let buttons = parts[1..(parts.len() - 1)]
				.iter()
				.map(|part| {
					let button_indices = part[1..(part.len() - 1)]
						.split(",")
						.map(|s| s.parse().unwrap())
						.collect::<Vec<usize>>();
					
					let mut button = 0u64;
					for button_index in button_indices {
						button |= 1u64 << button_index;
					}

					return button;
				})
				.collect();

			return (lights, buttons);
		})
		.collect();
}

fn find_fewest_presses(lights: &u64, buttons: &Vec<u64>) -> usize {
	let mut deque: VecDeque<(u64, usize)> = VecDeque::new();
	deque.push_back((0u64, 0));

	while deque.len() > 0 {
		let (state, presses) = deque.pop_front().unwrap();
		if state == *lights {
			return presses;
		}

		for button in buttons.iter() {
			let new_state = state.clone() ^ button;
			deque.push_back((new_state, presses + 1));
		}
	}

	return 0;
}

fn main() {
    let data = read_data();
	
	let mut result = 0;
	for (lights, buttons) in data.iter() {
		result += find_fewest_presses(lights, buttons);
	}
	
	println!("Result: {}", result); // 
}
