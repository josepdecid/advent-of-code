use std::collections::VecDeque;

fn read_data() -> Vec<Vec<char>>  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

    return data
        .lines()
        .map(|l| l.chars().collect())
        .collect();
}

fn get_cardinal_neighbors(field: &Vec<Vec<char>>, i: usize, j: usize, target: char) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if i > 0 { neighbors.push((i - 1, j)); }
    if j > 0 { neighbors.push((i, j - 1)); }
    if i < field.len() - 1 { neighbors.push((i + 1, j)); }
    if j < field[0].len() - 1 { neighbors.push((i, j + 1)); }

    return neighbors
        .into_iter()
        .filter(|n| field[n.0][n.1] == target)
        .collect();
}

fn calculate_crop_sizes(field: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> (u64, u64) {
    let mut area = 0;
    let mut perimeter = 0;
    
    let crop = field[x][y];
    visited[x][y] = true;

    let mut deque: VecDeque<(usize, usize)> = VecDeque::from([(x, y)]);
    while let Some((i, j)) = deque.pop_front() {
        area += 1;

        let neighbors = get_cardinal_neighbors(field, i, j, crop);
        perimeter += (4 - neighbors.len()) as u64;
        for (n_i, n_j) in neighbors {
            if !visited[n_i][n_j] {
                visited[n_i][n_j] = true;
                deque.push_back((n_i, n_j));
            }
        }
    }

    return (area, perimeter);
}

fn calculate_fencing_prices(field: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> u64 {
    let mut result = 0;

    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if !visited[i][j] {
                let crop = field[i][j];
                println!("Processing Crop {}...", crop);
                let (area, perimeter) = calculate_crop_sizes(field, visited, i, j);
                result += area * perimeter;
                println!("Processed Crop {}: Area {}, Perimeter {}", crop, area, perimeter);
            }            
        }
    }
    
    return result;
}

fn main() {
    let field = read_data();
    let mut visited = vec![vec![false; field[0].len()]; field.len()];
    
    let result = calculate_fencing_prices(&field, &mut visited);
    
    println!("Result: {}", result);
}
