use std::collections::HashMap;

use crate::days::common::Solution;

pub struct Day03 {}

impl Day03 {
    pub fn new() -> Self {
        Day03 {}
    }
}

struct Blueprint {
    numbers: HashMap<(usize, usize, usize), u32>,
    symbols: HashMap<(usize, usize), char>,
}

impl Blueprint {
    pub fn new() -> Self {
        Blueprint {
            numbers: HashMap::new(),
            symbols: HashMap::new(),
        }
    }

    pub fn find_gear_ratios(&self) -> Vec<u32> {
        let mut ratios: Vec<u32> = Vec::new();
        for (&(x, y), &c) in self.symbols.iter() {
            let mut nums: Vec<u32> = Vec::new();
            if c != '*' {
                continue;
            }

            for (&(xs, xe, yn), &n) in self.numbers.iter() {
                if y > yn + 1 || (yn > 0 && y < yn - 1) {
                    continue;
                }

                if x >= xs.saturating_sub(1) && x <= xe {
                    nums.push(n);
                    if nums.len() >= 2 {
                        break;
                    }
                }
            }
            if nums.len() >= 2 {
                ratios.push(nums.iter().fold(1u32, |a, c| a * c));
            }
        }
        ratios
    }

    pub fn find_symbol_neighbours(&self) -> Vec<u32> {
        let mut neighbours: Vec<u32> = Vec::new();
        for (&(xs, xe, y), &n) in self.numbers.iter() {
            if y > 0 && xs > 0 && self.symbols.contains_key(&(xs - 1, y - 1)) {
                neighbours.push(n);
            } else if xs > 0 && self.symbols.contains_key(&(xs - 1, y)) {
                neighbours.push(n);
            } else if xs > 0 && self.symbols.contains_key(&(xs - 1, y + 1)) {
                neighbours.push(n);
            } else if y > 0 && self.symbols.contains_key(&(xe, y - 1)) {
                neighbours.push(n);
            } else if self.symbols.contains_key(&(xe, y)) {
                neighbours.push(n);
            } else if self.symbols.contains_key(&(xe, y + 1)) {
                neighbours.push(n);
            } else {
                for x in xs..xe {
                    if y > 0 && self.symbols.contains_key(&(x, y - 1)) {
                        neighbours.push(n);
                        break;
                    } else if self.symbols.contains_key(&(x, y + 1)) {
                        neighbours.push(n);
                        break;
                    }
                }
            }
        }
        neighbours
    }
}

fn part1(input: &String) -> Result<i32, String> {
    let mut bp = Blueprint::new();
    let rows = input.trim().split("\n");
    for (y, line) in rows.enumerate() {
        let width = line.len();
        let mut x: usize = 0;
        while x < width {
            let c = line.chars().nth(x).unwrap();
            if c.is_numeric() {
                let mut num = String::new();
                let mut iter = line.chars().skip(x);
                let mut i = iter.next();
                while i.is_some() && i.unwrap().is_numeric() {
                    num.push_str(&i.unwrap().to_string());
                    i = iter.next();
                }
                bp.numbers
                    .insert((x, x + num.len(), y), num.parse::<u32>().unwrap());
                x += num.len() - 1;
            } else if c != '.' {
                bp.symbols.insert((x, y), c);
            }
            x += 1;
        }
    }
    let r = bp.find_symbol_neighbours().iter().sum::<u32>();
    Ok(r as i32)
}

fn part2(input: &String) -> Result<i32, String> {
    let mut bp = Blueprint::new();
    let rows = input.trim().split("\n");
    for (y, line) in rows.enumerate() {
        let width = line.len();
        let mut x: usize = 0;
        while x < width {
            let c = line.chars().nth(x).unwrap();
            if c.is_numeric() {
                let mut num = String::new();
                let mut iter = line.chars().skip(x);
                let mut i = iter.next();
                while i.is_some() && i.unwrap().is_numeric() {
                    num.push_str(&i.unwrap().to_string());
                    i = iter.next();
                }
                bp.numbers
                    .insert((x, x + num.len(), y), num.parse::<u32>().unwrap());
                x += num.len() - 1;
            } else if c == '*' {
                bp.symbols.insert((x, y), c);
            }
            x += 1;
        }
    }

    let r = bp.find_gear_ratios().iter().sum::<u32>();
    Ok(r as i32)
}

impl Solution for Day03 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        part1(input)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        part2(input)
    }
}
