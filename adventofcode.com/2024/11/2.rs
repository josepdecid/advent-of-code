const ITERATIONS: u8 = 75;

fn read_data() -> Vec<u64> {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

    return data
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
}

fn rune_blink(value: u64, iteration: u8) -> u64 {
    if iteration == ITERATIONS {
        return 1;
    }

    let next_iteration = iteration + 1;

    if value == 0 {
        return rune_blink(1, next_iteration);
    }

    let number_of_digits = value.to_string().len() as u32;
    if number_of_digits % 2 == 0 {
        let divident = 10_u64.pow(number_of_digits / 2);
        return rune_blink(value / divident, next_iteration)
            + rune_blink(value % divident, next_iteration);
    }

    return rune_blink(value * 2024, next_iteration);
}

fn main() {
    let data = read_data();
    let result = data.iter().map(|v| rune_blink(*v, 0)).sum::<u64>();
    println!("Result: {}", result);
}
