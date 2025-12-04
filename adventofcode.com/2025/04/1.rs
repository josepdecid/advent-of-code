type Grid = Vec<Vec<bool>>;

fn read_data() -> Grid  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);
    
    return data
        .lines()
        .map(|line| line.chars()
            .map(|c| c == '@')
            .collect())
        .collect();
}

fn has_less_than_four_paper_neighbors(grid: &Grid, i: usize, j: usize) -> bool {
	let mut count = 0;
	for offset_i in -1..=1 {
        for offset_j in -1..=1 {
			// Discard same position
            if offset_i == 0 && offset_j == 0 {
                continue;
            }

            let new_i = i as isize + offset_i;
			let new_j = j as isize + offset_j;
			
			// Discard elements out of bounds
			let max_i = (grid.len() - 1) as isize;
			let max_j = (grid[0].len() - 1) as isize;
			if new_i < 0 || new_j < 0 || new_i > max_i || new_j > max_j {
				continue;
			}

			// Discard non-paper positions
			let has_paper = grid[new_i as usize][new_j as usize];
			if !has_paper {
				continue;
			}

			count += 1;
			if count == 4 {
				return false;
			}
        }
    }
	
	return true;
}

fn main() {
    let grid = read_data();
    
	let mut count = 0;
	for i in 0..grid.len() {
		for j in 0..grid[0].len() {
			if grid[i][j] && has_less_than_four_paper_neighbors(&grid, i, j) {
				count += 1;
			}
		}
	}
    
	println!("Result: {}", count); // 1411
}
