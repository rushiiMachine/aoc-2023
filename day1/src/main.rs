use std::collections::HashMap;

use regex::Regex;

fn part1() {
    let input = include_str!("./part1.txt");
    let mut total = 0;

    for line in input.lines() {
        let mut digits = line.chars().filter(char::is_ascii_digit);
        let first = digits.next().unwrap();
        let last = digits.clone().last().unwrap_or_else(|| first);

        total += format!("{first}{last}").parse::<u32>().unwrap();
    }

    println!("{}", total);
}

fn part2() {
    let input = include_str!("./part2.txt");
    let regex = Regex::new(r#"(?<digit>\d)|(?<phrase>one|two|three|four|five|six|seven|eight|nine)"#).unwrap();
    let phrases_values = HashMap::from([
        ("one", 1u8),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut total = 0u32;
    for line in input.lines() {
        let mut values = regex
            .captures_iter(line)
            .filter_map(|capture| {
                if let Some(digit) = capture.name("digit") {
                    Some(digit.as_str().parse::<u8>().unwrap())
                } else if let Some(phrase) = capture.name("phrase") {
                    Some(*phrases_values.get(phrase.as_str()).unwrap())
                } else {
                    None
                }
            });

        total += format!("{}{}", values.next().unwrap(), values.last().unwrap())
            .parse::<u32>().unwrap();
    }

    println!("{}", total);
}

fn main() {
    part1();
    part2();
}
