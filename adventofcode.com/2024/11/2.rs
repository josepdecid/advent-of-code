use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct CacheKey {
    value: u64,
    iteration: u8,
}

const ITERATIONS: u8 = 75;

fn read_data() -> Vec<u64> {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

    return data
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
}

fn rune_blink(value: u64, iteration: u8, cache: &mut HashMap<CacheKey, u64>) -> u64 {
    let key = CacheKey { value, iteration };
    if let Some(&result) = cache.get(&key) {
        return result;
    }

    if iteration == ITERATIONS {
        cache.insert(key, 1);
        return 1;
    }

    let next_iteration = iteration + 1;
    let result = match value {
        0 => rune_blink(1, next_iteration, cache),
        v => {
            let number_of_digits = v.to_string().len() as u32;
            if number_of_digits % 2 == 0 {
                let divident = 10_u64.pow(number_of_digits / 2);
                rune_blink(v / divident, next_iteration, cache) + rune_blink(v % divident, next_iteration, cache)
            } else {
                rune_blink(v * 2024, next_iteration, cache)
            }
        }
    };

    cache.insert(key, result);
    return result;
}

fn main() {
    let data = read_data();

    let mut cache = HashMap::new();
    let result = data.iter()
        .map(|v| rune_blink(*v, 0, &mut cache))
        .sum::<u64>();
    
    println!("Result: {}", result);
}
