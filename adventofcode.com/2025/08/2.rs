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

fn build_circuits(distances: &Vec<(usize, usize, i64)>, num_boxes: usize) -> (usize, usize, i64) {
	let mut box_to_circuit = (0..num_boxes).collect::<Vec<usize>>();
	
	let mut i = 0;
	let mut num_circuits = num_boxes;
	while num_circuits > 2 {
		let a_circuit = box_to_circuit[distances[i].0];
		let b_circuit = box_to_circuit[distances[i].1];
		i += 1;
			
		// If A and B are not in the same circuit, assign circuit[A] to all boxes in circuit[B].
		if a_circuit != b_circuit {
			num_circuits -= 1;
			for p in 0..num_boxes {
				if box_to_circuit[p] == b_circuit {
					box_to_circuit[p] = a_circuit;
				}
			}
		}
	}
	
	// From the remaining shortest distances, find the first pair of boxes that do not belong to
	// the same circuit. As at this box there are exactly 2 circuits, it will always find one.
	return *distances[i..]
		.iter()
		.find(|(a, b, _)| box_to_circuit[*a] != box_to_circuit[*b])
		.unwrap()
}

fn main() {
    let boxes = read_data();
	let distances = compute_sorted_distances(&boxes);
	let (a, b, _) = build_circuits(&distances, boxes.len());
	
	let result = boxes[a].0 * boxes[b].0;
	println!("Result: {}", result); // 1474050600
}
