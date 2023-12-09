use std::{cmp::Ordering, collections::HashMap};

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_test() {
        let input = include_str!("test_input.txt");

        let (output, time_taken) = common::time_fn(part1, input);
        println!("Got '{output}' in {time_taken:#?}");
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test_input.txt");

        let (output, time_taken) = common::time_fn(part2, input);
        println!("Got '{output}' in {time_taken:#?}");
    }
}

fn card_strength(card: char) -> u32 {
    match card {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        c => panic!("dude {c} isn't a real card"),
    }
}

fn card_strength2(card: char) -> u32 {
    match card {
        'J' => 0,
        c => card_strength(c),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    fn new(cards: &str, joker: bool) -> Self {
        let mut card_counts_map = cards.chars().fold(HashMap::new(), |mut map, c| {
            let current = map.get(&c).unwrap_or(&0u8);
            map.insert(c, current + 1);
            map
        });

        if joker && card_counts_map.get(&'J').is_some_and(|n| n < &5) {
            if let Some(joker_count) = card_counts_map.remove(&'J') {
                let (highest_key, highest_value) =
                    card_counts_map.iter().max_by_key(|entry| entry.1).unwrap();
                card_counts_map.insert(*highest_key, highest_value + joker_count);
            }
        }

        let mut card_counts: Vec<(u8, char)> =
            card_counts_map.iter().map(|(a, b)| (*b, *a)).collect();
        card_counts.sort();
        card_counts.reverse();

        let highest_match = card_counts[0].0;
        if highest_match == 5 {
            Self::FiveOfAKind
        } else if highest_match == 4 {
            Self::FourOfAKind
        } else if highest_match == 3 {
            if card_counts[1].0 == 2 {
                Self::FullHouse
            } else {
                Self::ThreeOfAKind
            }
        } else if highest_match == 2 {
            if card_counts[1].0 == 2 {
                Self::TwoPair
            } else {
                Self::OnePair
            }
        } else {
            Self::HighCard
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    kind: HandKind,
    joker: bool,
}

impl Hand {
    fn new(cards: String, joker: bool) -> Self {
        let kind = HandKind::new(&cards, joker);
        Self { cards, kind, joker }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let kind_ordering = self.kind.cmp(&other.kind);
        match kind_ordering {
            Ordering::Equal => {
                if let Ordering::Equal = self.kind.cmp(&other.kind) {
                    for (a, b) in self.cards.chars().zip(other.cards.chars()) {
                        let (a, b) = if self.joker {
                            (card_strength2(a), card_strength2(b))
                        } else {
                            (card_strength(a), card_strength(b))
                        };
                        match a.cmp(&b) {
                            Ordering::Equal => {}
                            ord => {
                                return ord;
                            }
                        }
                    }
                }
                Ordering::Equal
            }
            ord => ord,
        }
    }
}

fn parse_line(line: &str, joker: bool) -> (Hand, u32) {
    let parts: Vec<&str> = line.split_whitespace().take(2).collect();
    (
        Hand::new(parts[0].to_string(), joker),
        parts[1].parse().unwrap(),
    )
}

fn part1(input: &str) -> u32 {
    let mut hands: Vec<(Hand, u32)> = input.lines().map(|line| parse_line(line, false)).collect();
    hands.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));
    let total = hands
        .into_iter()
        .enumerate()
        .fold(0, |total, (rank, (_, bid))| total + bid * (rank as u32 + 1));
    total
}

fn part2(input: &str) -> u32 {
    let mut hands: Vec<(Hand, u32)> = input.lines().map(|line| parse_line(line, true)).collect();
    hands.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));
    let total = hands
        .into_iter()
        .enumerate()
        .fold(0, |total, (rank, (_, bid))| total + bid * (rank as u32 + 1));
    total
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let part: u8 = args.get(1).unwrap().parse().unwrap();
    let input = include_str!("input.txt");
    if part == 1 {
        let (output, time_taken) = common::time_fn(part1, input);
        println!("Got '{output}' in {time_taken:#?}");
    } else if part == 2 {
        let (output, time_taken) = common::time_fn(part2, input);
        println!("Got '{output}' in {time_taken:#?}");
    }
}
