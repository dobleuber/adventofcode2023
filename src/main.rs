use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("{}", day2().unwrap());
}

fn day2() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/2/input.txt")?;

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    let result: Vec<_> = contents
        .lines()
        .map(|line| {
            let mut line_iter = line.split(':');
            let _game = line_iter
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .last()
                .unwrap();
            let balls = line_iter
                .next()
                .unwrap()
                .split(';')
                .fold([0, 0, 0], |mut acc, set| {
                    set.split(',').for_each(|ball| {
                        let mut ball_iter = ball.trim().split_ascii_whitespace();
                        let number = ball_iter.next().unwrap().parse::<u32>().unwrap();
                        let color = ball_iter.next().unwrap();
                        match color {
                            "red" if acc[0] < number => acc[0] = number,
                            "green" if acc[1] < number => acc[1] = number,
                            "blue" if acc[2] < number => acc[2] = number,
                            _ => (),
                        }
                    });
                    acc
                });
            balls
        })
        .inspect(|balls| println!("{:?}", balls))
        .map(|balls| balls.iter().product())
        .inspect(|product| println!("{}", product))
        .collect();

    println!("{:?}", result.iter().sum::<u32>());

    let result = result.iter().sum::<u32>();

    Ok(result)
}

fn _day1() -> Result<i32, std::io::Error> {
    let contents = open_file("./inputs/1/input.txt")?;
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

fn open_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
