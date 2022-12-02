use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;
use std::vec::Vec;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(pathname) = args.get(1) {
        let mut calories_list = Vec::new();
        let mut elf = 1;

        let f = File::open(pathname).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            match line {
                Ok(data) => {
                    if data.is_empty() {
                        elf = elf + 1;
                        let new_elf = (elf, 0);
                        calories_list.push(new_elf);
                    } else {
                        let parsed_calories = i32::from_str(&data).unwrap();

                        if let Some(last_elf) = calories_list.last_mut() {
                            last_elf.1 = last_elf.1 + parsed_calories;
                        } else {
                            let new_elf = (elf, parsed_calories);
                            calories_list.push(new_elf);
                        }
                   }
               },
               Err(error) => {
                    panic!("Cannot parse line: {:?}", error);
                }
            }
        }
    
        calories_list.sort_by(|a, b| a.1.cmp(&b.1));

        let mut total_calories = 0;
        for elf_pos in 1..4 {
            if let Some((_, calories)) = calories_list.pop() {
                println!("elf {} has {} calories", elf_pos, calories);
                total_calories = total_calories + calories;
            }
        }

        println!("Total Calories: {}", total_calories);
    }
}
