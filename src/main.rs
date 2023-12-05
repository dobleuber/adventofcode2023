// ignore dead code warnings
#![allow(dead_code)]
use regex::Regex;
use std::char;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("{}", day4_part2().unwrap());
}

fn day4_part2() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/4/input.txt")?;
    let mut cards: Vec<_> = contents
        .lines()
        .map(|line| {
            let line_iter = line.split(':');
            let cards = line_iter
                .last()
                .unwrap()
                .split('|')
                .map(|card| {
                    card.trim()
                        .split_ascii_whitespace()
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            (cards[0].clone(), cards[1].clone(), 1)
        })
        .collect();

    let mut index = 0;
    loop {
        let card = cards.get(index).unwrap().clone();
        let wins = card.0.iter().filter(|&n| card.1.contains(n)).count();

        for wins in (index + 1)..=index + wins {
            if let Some((_, _, ref mut count)) = cards.get_mut(wins) {
                *count += card.2;
            }
        }

        if index == cards.len() - 1 {
            break;
        }

        index += 1;
    }

    let results: Vec<_> = cards.iter().map(|(_, _, count)| *count).collect();

    Ok(results.iter().sum::<u32>())
}

fn day4_part1() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/4/input.txt")?;
    // every line looks like this:
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let result: Vec<_> = contents
        .lines()
        .map(|line| {
            let mut line_iter = line.split(':');
            line_iter.next().unwrap();
            let cards = line_iter
                .next()
                .unwrap()
                .split('|')
                .map(|card| {
                    card.trim()
                        .split_ascii_whitespace()
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            (cards[0].clone(), cards[1].clone())
        })
        .map(|(wining_numbers, game_numbers)| {
            let total_winning_numbers: Vec<u32> = wining_numbers
                .iter()
                .filter(|&wn| game_numbers.contains(wn))
                .cloned()
                .collect();
            if total_winning_numbers.is_empty() {
                0
            } else {
                1 << total_winning_numbers.len().checked_sub(1).unwrap()
            }
        })
        .inspect(|line| {
            dbg!(line);
        })
        .collect();

    Ok(result.iter().sum::<u32>())
}

fn day3_part2() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/3/input.txt")?;

    // convert to matrix
    let schematic_matrix: Vec<_> = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut schematic_numbers: Vec<u32> = vec![];
    let mut i = 0;

    while i < schematic_matrix.len() {
        let line = schematic_matrix.get(i).unwrap();
        let mut j = 0;
        while j < line.len() {
            let char = line.get(j).unwrap();
            if is_gear(*char) {
                if let Some(number) = is_valid_gear(&schematic_matrix, i, j) {
                    schematic_numbers.push(number);
                }
            }
            j += 1;
        }
        i += 1;
    }

    println!("{:?}", schematic_numbers);

    Ok(schematic_numbers.iter().sum::<u32>())
}

fn is_gear(char: char) -> bool {
    char == '*'
}

fn is_valid_gear(schematic_matrix: &Vec<Vec<char>>, i: usize, j: usize) -> Option<u32> {
    let lower_i = i.saturating_sub(1);
    let upper_i = schematic_matrix.len().saturating_sub(1).min(i + 1);
    let lower_j = j.saturating_sub(1);
    let upper_j = schematic_matrix
        .get(i)
        .unwrap()
        .len()
        .saturating_sub(1)
        .min(j + 1);
    let mut gear_numbers: Vec<u32> = vec![];
    for i in lower_i..=upper_i {
        let mut j = lower_j;
        while j <= upper_j {
            let char = schematic_matrix.get(i).unwrap().get(j).unwrap();
            if char.is_ascii_digit() {
                let whole_number = find_whole_number(schematic_matrix, i, &mut j);
                gear_numbers.push(whole_number);
            }
            j += 1;
        }
    }

    print!("{:?} ", gear_numbers);

    if gear_numbers.len() == 2 {
        Some(gear_numbers.iter().product())
    } else {
        None
    }
}

fn _day3() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/3/input.txt")?;

    // convert to matrix
    let schematic_matrix: Vec<_> = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut schematic_numbers: Vec<u32> = vec![];
    let mut i = 0;

    while i < schematic_matrix.len() {
        let line = schematic_matrix.get(i).unwrap();
        let mut j = 0;
        while j < line.len() {
            let char = line.get(j).unwrap();
            if char.is_ascii_digit() && number_is_valid(&schematic_matrix, i, j) {
                let whole_number = find_whole_number(&schematic_matrix, i, &mut j);
                schematic_numbers.push(whole_number);
            }
            j += 1;
        }
        i += 1;
    }

    println!("{:?}", schematic_numbers);

    Ok(schematic_numbers.iter().sum::<u32>())
}

fn number_is_valid(schematic_matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let lower_i = i.saturating_sub(1);
    let upper_i = schematic_matrix.len().saturating_sub(1).min(i + 1);
    let lower_j = j.saturating_sub(1);
    let upper_j = schematic_matrix
        .get(i)
        .unwrap()
        .len()
        .saturating_sub(1)
        .min(j + 1);
    for i in lower_i..=upper_i {
        for j in lower_j..=upper_j {
            let char = schematic_matrix.get(i).unwrap().get(j).unwrap();
            if is_symbol(*char) {
                return true;
            }
        }
    }

    false
}

fn is_symbol(char: char) -> bool {
    char != '.' && !char.is_alphanumeric() && !char.is_ascii_whitespace()
}

fn find_whole_number(schematic_matrix: &Vec<Vec<char>>, i: usize, j: &mut usize) -> u32 {
    let mut number_start_index = *j;

    let line = schematic_matrix.get(i).unwrap();

    loop {
        let temp_index = number_start_index.saturating_sub(1);
        let char = line.get(temp_index).unwrap();
        if char.is_ascii_digit() {
            number_start_index = temp_index;
        } else {
            break;
        }

        if temp_index == 0 {
            break;
        }
    }

    loop {
        let temp_index = *j + 1;
        let char = line.get(temp_index).unwrap();
        if char.is_ascii_digit() {
            *j = temp_index;
        } else {
            break;
        }

        if temp_index == line.len() - 1 {
            break;
        }
    }

    let number: String = line
        .iter()
        .skip(number_start_index)
        .take(*j - number_start_index + 1)
        .collect();

    number.parse::<u32>().unwrap()
}

fn _day2() -> Result<u32, std::io::Error> {
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
