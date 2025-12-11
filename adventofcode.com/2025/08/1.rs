const ITERATIONS: usize = 1000;
const TOP_CIRCUITS: usize = 3;

fn read_data() -> Vec<(i64, i64, i64)> {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

	return data
		.lines()
		.map(|line| {
			let coords = line
				.split(",")
				.map(|v| v.parse().unwrap())
				.collect::<Vec<i64>>();
			return (coords[0], coords[1], coords[2]);
		})
		.collect();
}

fn compute_sorted_distances(boxes: &Vec<(i64, i64, i64)>) -> Vec<(usize, usize, i64)> {
	let n = boxes.len();
	let mut distances = Vec::new();
	
	for a in 0..n {
		for b in (a + 1)..n {
			let dx = boxes[a].0 - boxes[b].0;
			let dy = boxes[a].1 - boxes[b].1;
			let dz = boxes[a].2 - boxes[b].2;
			
			let distance = dx * dx + dy * dy + dz * dz;
			distances.push((a, b, distance));
		}
	}

	distances.sort_by(|a, b| a.2.cmp(&b.2));
	return distances;
}

fn build_circuits(distances: &Vec<(usize, usize, i64)>, num_boxes: usize) -> Vec<usize> {
	let mut box_to_circuit = (0..num_boxes).collect::<Vec<usize>>();

	for i in 0..ITERATIONS {
		let a_circuit = box_to_circuit[distances[i].0];
		let b_circuit = box_to_circuit[distances[i].1];
		if a_circuit == b_circuit {
			continue;
		}

		for p in 0..num_boxes {
			if box_to_circuit[p] == b_circuit {
				box_to_circuit[p] = a_circuit;
			}
		}
	}

	let mut count_per_circuit = vec![0; num_boxes];
	for circuit in box_to_circuit {
		count_per_circuit[circuit] += 1;
	}
	
	count_per_circuit.sort_by(|a, b| b.cmp(a));
	return count_per_circuit;
}

fn main() {
    let boxes = read_data();
	let distances = compute_sorted_distances(&boxes);
	let circuit_sizes = build_circuits(&distances, boxes.len());
	
	let result = circuit_sizes.iter().take(TOP_CIRCUITS).product::<usize>();
	println!("Result: {}", result); // 52668
}
