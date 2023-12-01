use std::collections::HashMap;

use regex::Regex;

fn part1() {
    let input = include_str!("./input.txt");
    let mut total = 0u32;

    for line in input.lines() {
        let first = line.chars().into_iter().find(|char| char.is_ascii_digit()).unwrap();
        let last = line.chars().into_iter().rfind(|char| char.is_ascii_digit()).unwrap();

        total += format!("{first}{last}").parse::<u32>().unwrap();
    }

    println!("{}", total);
}

fn part2() {
    let input = include_str!("./input.txt");
    let mut total = 0u32;

    let regex = Regex::new(r#"\d|one|two|three|four|five|six|seven|eight|nine"#).unwrap();

    let phrases_values = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for line in input.lines() {
        let mut results = Vec::new();
        for (full_match, []) in regex.captures_iter(line).map(|c| c.extract()) {
            results.push(full_match);
        }

        let first = if results.first().unwrap().chars().into_iter().next().unwrap().is_ascii_digit() {
            results.first().unwrap().to_string()
        } else {
            phrases_values.get(results.first().unwrap()).unwrap().to_string()
        };
        let last = if results.last().unwrap().chars().into_iter().next().unwrap().is_ascii_digit() {
            results.last().unwrap().to_string()
        } else {
            phrases_values.get(results.last().unwrap()).unwrap().to_string()
        };

        total += format!("{first}{last}").parse::<u32>().unwrap();
    }

    println!("{}", total);
}

fn main() {
    part1();
    part2();
}
