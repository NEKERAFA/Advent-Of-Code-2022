use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::{env, fs::File, io::BufRead, io::BufReader};

type Part = fn(BufReader<File>, HashMap<char, u32>) -> u32;

fn get_priorities() -> HashMap<char, u32> {
    let initial_lower: u32 = 'a' as u32;
    let initial_upper: u32 = 'A' as u32;

    let mut priorities: HashMap<char, u32> = HashMap::new();
    for lower in 1..27 {
        let lower_char = char::from_u32(lower + initial_lower - 1).unwrap();
        priorities.insert(lower_char, lower);
    }

    for upper in 27..53 {
        let upper_char = char::from_u32(upper + initial_upper - 27).unwrap();
        priorities.insert(upper_char, upper);
    }

    return priorities;
}

fn string_to_set(string: &str) -> HashSet<char> {
    let mut str_set: HashSet<char> = HashSet::new();
    for character in string.chars() {
        str_set.insert(character);
    }

    return str_set;
}

fn get_common(list: Vec<&str>) -> char {
    let mut common_chars = string_to_set(list[0]);

    for set_index in 1..list.len() {
        let current = string_to_set(list[set_index]);
        common_chars = common_chars.intersection(&current).copied().collect();
    }

    return Vec::from_iter(common_chars)[0];
}

fn part1(reader: BufReader<File>, priorities: HashMap<char, u32>) -> u32 {
    let mut score = 0;

    for line in reader.lines() {
        let data = line.unwrap();

        /*
        let first_part = data.get(0..data.len() / 2).unwrap();
        print!("{}", first_part);

        let mut first_set: HashSet<char> = HashSet::new();
        for character in first_part.chars() {
            first_set.insert(character);
        }

        let second_part = data.get(data.len() / 2..data.len()).unwrap();
        print!(" {}", second_part);

        let mut second_set: HashSet<char> = HashSet::new();
        for character in second_part.chars() {
            second_set.insert(character);
        }

        let mut intersect = first_set.intersection(&second_set);
        let item = intersect.nth(0).unwrap();
        print!(": {}", item);


        if let Some(priority) = priorities.get(item) {
            println!(" ({})", priority);
            score = score + priority;
        }
        */

        let first_part = data.get(0..data.len() / 2).unwrap();
        print!("{}", first_part);

        let second_part = data.get(data.len() / 2..data.len()).unwrap();
        print!(" {}", second_part);

        let rucksacks = vec![first_part, second_part];
        let item = get_common(rucksacks);
        print!(": {}", item);

        if let Some(priority) = priorities.get(&item) {
            println!(" ({})", priority);
            score = score + priority;
        }
    }

    return score;
}

fn part2(reader: BufReader<File>, priorities: HashMap<char, u32>) -> u32 {
    let mut score = 0;
    let mut rucksacks: Vec<String> = Vec::new();
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    for line in lines {
        println!("{}", line);
        rucksacks.push(line);

        if rucksacks.len() == 3 {
            let group = vec![
                rucksacks[0].as_str(),
                rucksacks[1].as_str(),
                rucksacks[2].as_str(),
            ];
            let item = get_common(group);
            print!("{}", item);

            if let Some(priority) = priorities.get(&item) {
                println!(" {}", priority);
                score = score + priority;
            }

            rucksacks.clear();
        }
    }

    return score;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(pathname) = args.get(1) {
        let file = File::open(pathname).unwrap();
        let reader = BufReader::new(file);

        let mut func: Part = part1;
        if let Some(part) = args.get(2) {
            let part_parsed = i32::from_str(&part).unwrap();

            if part_parsed == 2 {
                func = part2;
            }
        }

        let priorities = get_priorities();
        let total_priority = func(reader, priorities);
        println!("Total Priority: {}", total_priority);
    }
}
