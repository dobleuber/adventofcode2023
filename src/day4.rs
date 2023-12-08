use crate::utils::open_file;

pub fn day4_part2() -> Result<u32, std::io::Error> {
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

pub fn day4_part1() -> Result<u32, std::io::Error> {
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
