use crate::days::common::Solution;

pub struct Day07 {}

impl Day07 {
    pub fn new() -> Self {
        Day07 {}
    }
}

struct Hand {
    cards: HashMap<u8, usize>
}

impl From<&Vec<u8>> for Hand {
    pub fn from(cards: &Vec<u8>) -> Self {
        Hand {
            cards: cards
                .iter()
                .fold(
                    HashMap::new(),
                    |mut cnt, c| {
                        *cnt.entry(c).or_insert(0) += 1;
                        cnt
                    }
                )
        }
    }
}

fn to_int(c: &Char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        &_ => c.parse::<u8>().unwrap()
    }
}

pub fn sort_hands(hands: &mut Vec<Hand

fn parse(input: &String) -> () {
    let hands = input
        .lines()
        .map(|l| {
            let cards = l.split(" ").nth(0).unwrap().chars().map(|c| to_int(c)).collect::<Vec<u8>>();
            let cards: Hand = &cards.into();
            let bid = l.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
        })
        .collect::<Vec<Hand, u32>>();
    // Build a hashset per hand
    // if len == 5, high card
    // if len == 4, pair
    // if len <= 3, build a hashmap/counter
    // if len == 3 and max_count == 2, two pairs
    // if len == 3 and max_count == 3, three of a kind
    // if len == 2 and max_count == 3, full
    // if len == 2 and max_count == 4, 4ok
    // if len == 1, 5k
}

impl Solution for Day07 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        Ok(5)
    }
}
