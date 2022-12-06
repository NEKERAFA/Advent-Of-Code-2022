use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::vec::Vec;
use std::str::FromStr;
use std::env;

const OTHER_ROCK: char = 'A';
const OTHER_PAPER: char = 'B';
const OTHER_SCISSORS: char = 'C';

const MY_ROCK: char = 'X';
const MY_PAPER: char = 'Y';
const MY_SCISSORS: char = 'Z';

fn get_shape_score(action: char) -> i32 {
    return match action {
        MY_ROCK => 1,
        MY_PAPER => 2,
        MY_SCISSORS => 3,
        _ => 0
    }
}

fn get_outcome_rock(action: char) -> i32 {
    return match action {
        MY_ROCK => 3,
        MY_PAPER => 6,
        _ => 0
    }
}

fn get_outcome_paper(action: char) -> i32 {
    return match action {
        MY_PAPER => 3,
        MY_SCISSORS => 6,
        _ => 0
    }
}

fn get_outcome_scissors(action: char) -> i32 {
    return match action {
        MY_SCISSORS => 3,
        MY_ROCK => 6,
        _ => 0
    }
}

fn part1(reader: BufReader<File>) -> i32 {
    let mut score = 0;

    for line in reader.lines() {
        match line {
            Ok(data) => {
                if let Some(other_action) = data.chars().nth(0) {
                    if let Some(me_action) = data.chars().nth(2) {
                        let shape_score = get_shape_score(me_action);
                        print!("{} + ", shape_score);

                        let outcome_score = match other_action {
                            OTHER_ROCK => get_outcome_rock(me_action),
                            OTHER_PAPER => get_outcome_paper(me_action),
                            OTHER_SCISSORS => get_outcome_scissors(me_action),
                            _ => 0
                        };

                        score = score + shape_score + outcome_score;
                        println!("{} = {} (total: {})", outcome_score, shape_score + outcome_score, score)
                    }
                }
            },
            Err(error) => {
                panic!("Cannot read line: {:?}", error);
            }
        }
    }

    return score;
}

const LOSE_RESULT: char = 'X';
const DRAW_RESULT: char = 'Y';
const WIN_RESULT: char = 'Z';

fn get_lose_action(other_action: char) -> char {
    return match other_action {
        OTHER_ROCK => MY_SCISSORS,
        OTHER_PAPER => MY_ROCK,
        OTHER_SCISSORS => MY_PAPER,
        _ => panic!("action not found: {}", other_action)
    }
}

fn get_draw_action(other_action: char) -> char {
    return match other_action {
        OTHER_ROCK => MY_ROCK,
        OTHER_PAPER => MY_PAPER,
        OTHER_SCISSORS => MY_SCISSORS,
        _ => panic!("action not found: {}", other_action)
    }
}

fn get_win_action(other_action: char) -> char {
    return match other_action {
        OTHER_ROCK => MY_PAPER,
        OTHER_PAPER => MY_SCISSORS,
        OTHER_SCISSORS => MY_ROCK,
        _ => panic!("action not found: {}", other_action)
    }
}

fn part2(reader: BufReader<File>) -> i32 {
    let mut score = 0;

    for line in reader.lines() {
        match line {
            Ok(data) => {
                if let Some(other_action) = data.chars().nth(0) {
                    if let Some(me_result) = data.chars().nth(2) {
                        let me_action = match me_result {
                            LOSE_RESULT => get_lose_action(other_action),
                            DRAW_RESULT => get_draw_action(other_action),
                            WIN_RESULT => get_win_action(other_action),
                            _ => panic!("Result not found: {}", me_result)
                        };

                        let shape_score = get_shape_score(me_action);
                        print!("{} + ", shape_score);

                        let outcome_score = match other_action {
                            OTHER_ROCK => get_outcome_rock(me_action),
                            OTHER_PAPER => get_outcome_paper(me_action),
                            OTHER_SCISSORS => get_outcome_scissors(me_action),
                            _ => 0
                        };

                        score = score + shape_score + outcome_score;
                        println!("{} = {} (total: {})", outcome_score, shape_score + outcome_score, score)
                    }
                }
            },
            Err(error) => {
                panic!("Cannot read line: {:?}", error);
            }
        }
    }

    return score;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(pathname) = args.get(1) {
        let f = File::open(pathname).unwrap();
        let reader = BufReader::new(f);

        let mut score = 0;
        if let Some(part) = args.get(2) {
            let part_parsed = i32::from_str(&part).unwrap();

            if part_parsed == 1 {
                score = part1(reader);
            } else if part_parsed == 2 {
                score = part2(reader);
            }
        } else {
            score = part1(reader);
        }

        println!("Score: {}", score);
    }
}
