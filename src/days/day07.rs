use crate::days::common::Solution;

use std::collections::HashMap;

pub struct Day07 {}

impl Day07 {
    pub fn new() -> Self {
        Day07 {}
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Card {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HandKind {
    HighCard,
    Pair,
    TwoPairs,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    kind: HandKind,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            'J' => Card::Joker,
            _ => panic!("Bananas"),
        }
    }
}

impl Hand {
    pub fn new(cards: Vec<char>) -> Self {
        let kind = Hand::get_kind(&cards);
        let cards = cards.into_iter().map(Card::from).collect();
        Hand { cards, kind }
    }

    pub fn get_kind(cards: &[char]) -> HandKind {
        let mut map = cards
            .iter()
            .fold(HashMap::<char, u64>::new(), |mut acc, x| {
                *acc.entry(*x).or_insert(0) += 1;
                acc
            });
        // add count of j to 2nd most common and remove j from map
        let non_j_highest_card = map.iter().fold(None, |mut acc, (card, count)| {
            if *card != 'J' {
                acc = std::cmp::max(acc, Some((*count, *card)));
            }
            acc
        });

        if let Some(non_j_highest_card) = non_j_highest_card {
            *map.entry(non_j_highest_card.1).or_default() += *map.get(&'J').unwrap_or(&0);
            map.remove(&'J');
        }

        if map.values().any(|v| *v == 5) {
            HandKind::Five
        } else if map.values().any(|v| *v == 4) {
            HandKind::Four
        } else if map.values().any(|v| *v == 3) {
            if map.values().any(|v| *v == 2) {
                HandKind::Full
            } else {
                HandKind::Three
            }
        } else if map.values().filter(|v| **v == 2).count() == 2 {
            HandKind::TwoPairs
        } else if map.values().filter(|v| **v == 2).count() == 1 {
            HandKind::Pair
        } else {
            HandKind::HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.kind.cmp(&other.kind) {
            std::cmp::Ordering::Equal => Some(
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .find_map(|(m, o)| match m.cmp(o) {
                        std::cmp::Ordering::Equal => None,
                        order => Some(order.reverse()),
                    })
                    .unwrap(),
            ),
            same => Some(same),
        }
    }
}

pub fn solve(input: &String) -> u64 {
    let mut hands = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let cards = line
                .split(' ')
                .nth(0)
                .unwrap()
                .chars()
                .collect::<Vec<char>>();
            let num = line.split(' ').nth(1).unwrap().parse::<u64>().unwrap();
            (Hand::new(cards), num)
        })
        .collect::<Vec<(Hand, u64)>>();
    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, num))| (i as u64 + 1) * num)
        .sum::<u64>()
}

impl Solution for Day07 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        let s = solve(input);
        println!("{}", &s);
        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        let s = solve(input);
        println!("{}", &s);
        Ok(5)
    }
}
