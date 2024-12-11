use std::collections::LinkedList;

fn read_data() -> LinkedList<u64>  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);

    return data
        .split_whitespace()
        .map(|v| v.parse().unwrap()).collect();
}

fn blink(data: &mut LinkedList<u64>) {
    let mut cursor = data.cursor_mut();

    while let Some(current) = cursor.current() {
        if *current == 0 {
            *current = 1;
            cursor.move_next();
            continue;
        }

        let number_of_digits = *current.to_string().len() as u32;
        if number_of_digits % 2 == 0 {
            let divident = 10_u64.pow(number_of_digits / 2);
            let value = *current;
            *current = data / divident;
            cursor.move_next();
            cursor.insert()
            data.insert(i + 1, data[i] % divident);
            data[i] = data[i] / divident;
            cursor.move_next();
            continue;
        }
    
        data[i] = data[i] * 2024;
        i += 1;
    }
}

fn main() {
    let mut data = read_data();
    for i in 0..25 {
        println!("Blink #{}", i);
        blink(&mut data);
    }

    println!("Result: {}", data.len());
}
