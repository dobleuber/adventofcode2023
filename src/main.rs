// ignore dead code warnings
#![allow(dead_code)]
use core::str::Lines;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

fn main() {
    let result = day7().unwrap();
    println!("{}", result);
}

fn day7() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/7/input.txt")?;
    let mut hands: Vec<_> = contents.lines().map(Hand::parse).collect();
    hands.sort();
    hands.iter().for_each(|hand| println!("{}", hand));

    let result = hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        acc + (hand.bid * (index as u32 + 1))
    });

    Ok(result)
}

#[derive(Debug, PartialEq, Eq)]
enum SetType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPairs,
    Pair,
    HighCard,
}

impl PartialOrd for SetType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SetType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Five, Self::Five)
            | (Self::Four, Self::Four)
            | (Self::Three, Self::Three)
            | (Self::HighCard, Self::HighCard)
            | (Self::Pair, Self::Pair) => Ordering::Equal,
            (Self::TwoPairs, Self::TwoPairs) | (Self::FullHouse, Self::FullHouse) => {
                Ordering::Equal
            }
            (Self::Five, _) => Ordering::Greater,
            (_, Self::Five) => Ordering::Less,

            (Self::Four, _) => Ordering::Greater,
            (_, Self::Four) => Ordering::Less,

            (Self::FullHouse, _) => Ordering::Greater,
            (_, Self::FullHouse) => Ordering::Less,

            (Self::Three, _) => Ordering::Greater,
            (_, Self::Three) => Ordering::Less,

            (Self::TwoPairs, _) => Ordering::Greater,
            (_, Self::TwoPairs) => Ordering::Less,

            (Self::Pair, _) => Ordering::Greater,
            (_, Self::Pair) => Ordering::Less,
        }
    }
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<u16>,
    bid: u32,
    set_type: SetType,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cards: Vec<_> = self
            .cards
            .iter()
            .map(|&card| match card {
                14 => 'A',
                13 => 'K',
                12 => 'Q',
                1 => 'J',
                10 => 'T',
                _ => card.to_string().chars().next().unwrap(),
            })
            .collect();
        let cards = cards
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{} - {:?}", cards, self.set_type)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.set_type.eq(&other.set_type) && self.cards.eq(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.set_type.cmp(&other.set_type).then_with(|| {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(&s, &o)| if s != o { Some(s.cmp(&o)) } else { None })
                .unwrap_or(Ordering::Equal)
        })
    }
}

impl Hand {
    fn parse(line: &str) -> Self {
        let mut line = line.split_ascii_whitespace();

        let cards = line
            .next()
            .unwrap()
            .chars()
            .map(|card| match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                _ => card.to_digit(10).unwrap() as u16,
            })
            .collect::<Vec<_>>();

        let bid = line.next().unwrap().parse::<u32>().unwrap();

        let counts: HashMap<_, _> = cards
            .iter()
            .fold(HashMap::new(), |mut acc, &card| {
                *acc.entry(card).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .collect();

        let mut counts: Vec<_> = counts.into_iter().collect();

        let j_count = counts
            .iter()
            .find(|(card, count)| card == &1 && *count < 5)
            .cloned();

        if let Some(j_count) = j_count {
            counts.retain(|(card, _)| card != &1);
            counts.sort_by(|a, b| {
                if a.1 == 1 {
                    Ordering::Less
                } else {
                    a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0))
                }
            });

            if let Some(last_element) = counts.last_mut() {
                last_element.1 += j_count.1;
            }
        }

        let mut counts: Vec<_> = counts.iter().filter(|&(_, count)| *count > 1).collect();

        counts.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

        println!("4 {:?}", counts);

        let set_type: SetType = match counts.len() {
            1 => match counts[0].1 {
                5 => SetType::Five,
                4 => SetType::Four,
                3 => SetType::Three,
                2 => SetType::Pair,
                _ => SetType::HighCard,
            },
            2 => match counts[1].1 {
                3 => SetType::FullHouse,
                2 => SetType::TwoPairs,
                _ => SetType::HighCard,
            },
            _ => SetType::HighCard,
        };

        Self {
            cards,
            bid,
            set_type,
        }
    }
}

fn day6_part2() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/6/input.txt")?;
    let times: Vec<_> = contents
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect();
    let distances: Vec<_> = contents
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect();

    let total_time = times.join("").parse::<u64>().unwrap();
    let total_distance = distances.join("").parse::<u64>().unwrap();

    println!("{:?}", total_time);
    println!("{:?}", total_distance);

    let ways = (1..total_time)
        .map(|i| (total_time - i) * i)
        .filter(|&md| md > total_distance)
        .collect::<Vec<_>>();

    Ok(ways.len() as u32)
}

fn day6_part1() -> Result<u32, std::io::Error> {
    let contents = open_file("./inputs/6/input.txt")?;
    let times: Vec<_> = contents
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|time| time.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<_> = contents
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|time| time.parse::<u32>().unwrap())
        .collect();

    let ways: Vec<_> = times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| {
            (1..t)
                .map(|i| (t - i) * i)
                .filter(|&md| md > d)
                .collect::<Vec<_>>()
        })
        .map(|w| w.len())
        .collect();

    let result = ways.iter().map(|&w| w as u32).product::<u32>();

    Ok(result)
}

#[derive(Debug)]
struct Field {
    // (index, value)
    ranges: Range<u32>,
}

impl Field {
    fn shift(&mut self, shift: &Shift) {
        let (destination, source, len) = (shift.0, shift.1, shift.2);
    }
}

#[derive(Debug)]
struct Shift(u32, u32, u32);

impl Shift {
    fn parse(line: &str) -> Option<Self> {
        if line.is_empty() {
            return None;
        }

        let mut line_iter = line.split_ascii_whitespace();
        let destination = line_iter.next().unwrap().parse::<u32>().unwrap();
        let source = line_iter.next().unwrap().parse::<u32>().unwrap();
        let count = line_iter.next().unwrap().parse::<u32>().unwrap();
        Some(Self(destination, source, count))
    }
}

#[derive(Debug)]
struct ShiftGroup {
    shifts: Vec<Shift>,
}

impl ShiftGroup {
    fn parse(lines: &mut Lines) -> Self {
        let mut lines = lines.peekable();
        while let Some(line) = lines.peek() {
            if line.contains("map:") {
                lines.next();
                break;
            }
            lines.next();
        }

        let shifts: Vec<_> = lines
            .map(Shift::parse)
            .take_while(|shift| shift.is_some())
            .map(|shift| shift.unwrap())
            .collect();
        Self { shifts }
    }
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
