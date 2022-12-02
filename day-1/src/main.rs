use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

fn main() {
    let mut elf = 1;
    let mut current_elf = 1;
    let mut max_calories = 0;
    let mut calories = 0;

    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        match line {
            Ok(data) => {
                if data.is_empty() {
                    if max_calories < calories {
                        max_calories = calories;
                        elf = current_elf;
                    }

                    current_elf = current_elf + 1;
                    calories = 0;
                } else {
                    let parsed_calories = i32::from_str(&data).unwrap();
                    calories = calories + parsed_calories;
                }
            },
            Err(error) => {
                panic!("Cannot parse line: {:?}", error);
            }
        }
    }

    println!("elf {} has {} calories", elf, max_calories);
}
