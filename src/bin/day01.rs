use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("No file found");

    part_1(&contents);
    part_2(&contents);
}

fn part_1(input: &String) {
    let result: u32 = input.lines().map(find_digits).sum();
    println!("result part 1: {}", result);
}

fn find_first_digit(mut iterator: impl Iterator<Item = char>) -> u32 {
    if let Some(dig) = iterator.find(|c| c.is_numeric()) {
        dig.to_digit(10).unwrap()
    } else {
        0
    }
}
fn find_digits(line: &str) -> u32 {
    let first_digit = find_first_digit(line.chars());
    let last_digit = find_first_digit(line.chars().rev());

    let digits = first_digit.to_string() + &last_digit.to_string();
    return digits.parse().unwrap();
}

fn get_matches(line: &str) -> Vec<&str> {
    let values = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string(),
        "8".to_string(),
        "9".to_string(),
    ];
    let mut index = 0;
    let mut matches_in_line: Vec<&str> = vec![];
    while index < line.len() {
        let mut end_index = line.len();

        while end_index > index {
            let slice = &line[index..end_index];
            if values.contains(&slice.to_string()) {
                matches_in_line.push(&slice);
            }
            end_index -= 1;
        }

        index += 1;
    }
    return matches_in_line;
}

fn get_value_from_matches(matches: Vec<&str>) -> u32 {
    let map: HashMap<_, _> = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into_iter()
    .collect();
    let (&first, &last) = (
        matches.iter().next().unwrap(),
        matches.iter().rev().next().unwrap(),
    );

    let first_digit = map.get(first).unwrap().to_string();
    let last_digit = map.get(last).unwrap().to_string();
    let digits = first_digit + &last_digit;
    return digits.parse().unwrap();
}

fn part_2(input: &String) {
    let result: u32 = input
        .lines()
        .map(get_matches)
        .map(get_value_from_matches)
        .sum();
    println!("Result part 2: {}", result);
}
