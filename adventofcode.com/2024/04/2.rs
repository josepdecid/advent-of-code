fn has_x_mas(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    return
        matrix[i][j] == 'A' &&
        (matrix[i-1][j-1] == 'M' && matrix[i+1][j+1] == 'S' || matrix[i-1][j-1] == 'S' && matrix[i+1][j+1] == 'M') &&
        (matrix[i-1][j+1] == 'M' && matrix[i+1][j-1] == 'S' || matrix[i-1][j+1] == 'S' && matrix[i+1][j-1] == 'M')
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

    let matrix: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count: usize = 0;
    for i in 1..(matrix.len() - 1) {
        for j in 1..(matrix[0].len() - 1) {
            if has_x_mas(&matrix, i, j) {
                count += 1;
            }
        }
    }

    println!("{}", count);  // 2048
}
