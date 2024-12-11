fn read_data() -> Vec<u64>  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

    return data
        .split_whitespace()
        .map(|v| v.parse().unwrap()).collect();
}

fn blink(data: &mut Vec<u64>) {
    let mut i = 0;
    while i < data.len() {
        if data[i] == 0 {
            data[i] = 1;
            i += 1;
            continue;
        }

        let number_of_digits = data[i].to_string().len() as u32;
        if number_of_digits % 2 == 0 {
            let divident = 10_u64.pow(number_of_digits / 2);
            data.insert(i + 1, data[i] % divident);
            data[i] = data[i] / divident;
            i += 2;
            continue;
        }
    
        data[i] = data[i] * 2024;
        i += 1;
    }
}

fn main() {
    let mut data = read_data();
    for i in 0..75 {
        println!("Blink #{}", i);
        blink(&mut data);
    }

    println!("Result: {}", data.len());
}
