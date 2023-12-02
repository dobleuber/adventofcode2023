use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("{}", day1().unwrap());
}

fn day1() -> Result<i32, std::io::Error> {
    let mut file = File::open("./inputs/1/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let result: Vec<_> = contents
        .lines()
        .map(map_line)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let first = line.first().unwrap();
            let last = line.last().unwrap_or(first);
            format!("{}{}", first, last)
        })
        .map(|number| number.parse::<i32>().unwrap())
        .collect();
    // for line in contents.lines() {
    //     let mapped_line = map_line(line);
    //     let first = mapped_line.first().unwrap();
    //     let last = mapped_line.last().unwrap_or(first);

    //     println!("{} => {}{}", line, first, last);
    // }

    Ok(result.iter().sum::<i32>())
}

fn map_line(line: &str) -> Vec<&str> {
    let re = Regex::new(r"((\d)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine))")
        .unwrap();

    let mut start = 0;
    let mut matches = vec![];

    while let Some(matched) = re.find(&line[start..]) {
        let number_matched = &line[start + matched.start()..start + matched.end()];
        matches.push(number_matched);
        start += matched.start() + 1; // Aumenta la posición de inicio para buscar solapamientos
    }
    matches
        .iter()
        .map(|&m| match m {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => m,
        })
        .collect::<Vec<_>>()
}
