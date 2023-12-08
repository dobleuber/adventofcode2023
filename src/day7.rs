use crate::utils::open_file;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

pub fn day7() -> Result<u32, std::io::Error> {
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
