// https://adventofcode.com/2024/day/10

fn read_data() -> Vec<Vec<char>> {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);
    
    return data
        .lines()
        .map(|line| line.chars().collect())
        .collect();
}

fn count_trailhead_scores(matrix: &Vec<Vec<char>>, i: usize, j: usize, target: char) -> u32 {
    if matrix[i][j] != target {
        return 0;
    }

    if matrix[i][j] == '9' {
        return 1;
    }
    
    let mut score = 0;
    let next_target = (target as u8 + 1) as char;

    if i > 0 {
        score += count_trailhead_scores(matrix, i - 1, j, next_target);
    }

    if i < matrix.len() - 1 {
        score += count_trailhead_scores(matrix, i + 1, j, next_target);
    }

    if j > 0 {
        score += count_trailhead_scores(matrix, i, j - 1, next_target);
    }

    if j < matrix[0].len() - 1 {
        score += count_trailhead_scores(matrix, i, j + 1, next_target);
    }

    return score;
}

fn count_matrix_scores(matrix: &Vec<Vec<char>>) -> u32 {
    let mut score = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            score += count_trailhead_scores(&matrix, i, j, '0');
        }
    }

    return score;
}

fn main() {
    let data = read_data();
    let result = count_matrix_scores(&data);
    println!("Result: {}", result);  // Result: 1541
}

