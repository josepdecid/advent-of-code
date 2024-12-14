fn count_xmas(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    if matrix[x][y] != 'X' {
        return 0;
    }

    let i = x as i32;
    let j = y as i32;

    let matches_in_direction = |offset_i: i32, offset_j: i32| -> bool {
        let limit_i = i + 3 * offset_i;
        let limit_j = j + 3 * offset_j;

        if
            limit_i < 0 || limit_i >= matrix.len() as i32 ||
            limit_j < 0 || limit_j >= matrix[0].len() as i32
        {
            return false;
        }

        return
            matrix[(i + offset_i) as usize][(j + offset_j) as usize] == 'M' &&
            matrix[(i + 2 * offset_i) as usize][(j + 2 * offset_j) as usize] == 'A' &&
            matrix[(i + 3 * offset_i) as usize][(j + 3 * offset_j) as usize] == 'S';
    };

    return [(0, 1), (0, -1), (1, 0), (1, -1), (1, 1), (-1, 0), (-1, -1), (-1, 1)]
        .iter()
        .map(|&(inc_i, inc_j)| matches_in_direction(inc_i, inc_j))
        .filter(|&x| x)
        .count();
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

    let matrix: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count: usize = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            count += count_xmas(&matrix, i, j);
        }
    }

    println!("{}", count);  // 2685
}
