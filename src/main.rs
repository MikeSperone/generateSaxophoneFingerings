use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    // println!("Hello, world!");
    let mut value: i32 = 0;
    let mut count: i32 = 0;
    let MAX_KEYS = 10;
    let mut file = OpenOptions::new()
        .append(true)
        .open("fingerings.txt")
        .unwrap();

    while value < 8388607 {
        value += 1;
        let binary_value = format!("{:023b}", value);
        if value.count_ones() > MAX_KEYS {
            continue;
        }
        let binary_array: Vec<i32> =
            binary_value.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

        if binary_array[7] == 1 && (binary_array[0] == 1 || binary_array[10] ==1) {
            continue;
        }
        if binary_array[18] == 1 && (binary_array[17] == 1 || binary_array[19] == 1) {
            continue;
        }
        if binary_array[19] == 1 && binary_array[17] == 1 {
            continue;
        }
        if binary_array[20] == 1 && (binary_array[17] == 1 || binary_array[18] == 1|| binary_array[19] == 1) {
            continue;
        }
        if (binary_array[1] == 1 || binary_array[3] == 1 || binary_array[4] == 1 || binary_array[5] == 1) && binary_array[8] == 1 {
            continue;
        }
        if (binary_array[3] == 1 || binary_array[4] == 1) && binary_array[17] == 1 {
            continue;
        }
        writeln!(file, "{}", binary_value).unwrap();
        count += 1;
    }
    println!("{} fingerings found", count);
}

