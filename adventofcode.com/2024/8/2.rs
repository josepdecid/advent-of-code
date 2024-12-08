type Matrix = Vec<Vec<char>>;

fn read_data() -> Matrix  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);
    
    return data
        .lines()
        .map(|line| line.chars().collect())
        .collect();
}

fn count_unique_antinodes(matrix: &Matrix) -> u32 {
    let mut antinodes_count = 0;
    let mut antinodes_matrix = vec![vec![false; matrix.len()]; matrix[0].len()];

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '.' {
                continue;
            }
                
            for i2 in 0..matrix.len() {
                for j2 in 0..matrix[0].len() {
                    if i == i2 && j == j2 || matrix[i][j] != matrix[i2][j2] {
                        continue;
                    }
                                      
                    let offset_i = i2 as i16 - i as i16;
                    let offset_j = j2 as i16 - j as i16;

                    let mut i3 = i2 as i16;
                    let mut j3 = j2 as i16;

                    while i3 >= 0 && i3 < matrix.len() as i16 && j3 >= 0 && j3 < matrix[0].len() as i16 {
                        if !antinodes_matrix[i3 as usize][j3 as usize] {
                            antinodes_count += 1;
                            antinodes_matrix[i3 as usize][j3 as usize] = true;
                        }

                        i3 += offset_i;
                        j3 += offset_j;
                    }
                }
            } 
        }
    }

    return antinodes_count;
}

fn main() {
    let matrix = read_data();
    let result = count_unique_antinodes(&matrix);
    println!("Result {}", result);  // Result: 1131
}

