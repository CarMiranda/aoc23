use std::collections::HashSet;

use crate::days::common::Solution;

pub struct Day18 {}

impl Day18 {
    pub fn new() -> Self {
        Day18 {}
    }
}

type Coordinates = (isize, isize);

#[derive(Debug, Eq, PartialEq, Hash)]
struct Border {
    start: Coordinates,
    end: Coordinates,
    color: (u8, u8, u8),
}

fn advance(coords: (isize, isize), dir: u8, length: usize) -> (isize, isize) {
    match dir {
        0 => (coords.0 + 1, coords.1),
        1 => (coords.0, coords.1 - 1),
        2 => (coords.0 - 1, coords.1),
        3 => (coords.0, coords.1 + 1),
        _ => panic!("Bananas")
    }
}

fn parse(input: &String) -> HashSet<Border> {
    let mut borders: HashSet<Border> = HashSet::new();
    let current_coords = (0, 0);
    for l in input.lines() {
        let d = match l.split(" ").nth(0).unwrap() {
            "R" => 0,
            "U" => 1,
            "L" => 2,
            "D" => 3,
            _ => panic!("Bananas"),
        };
        let length = l.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
        let color = l.split(" ").nth(2).unwrap().strip_prefix("(#").unwrap().strip_suffix(")").unwrap().to_string();
        let color: Vec<u8> = (0..color.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&color[i..i+2], 16).unwrap())
            .collect();
        let color = (color[0], color[1], color[2]);
        let new_coords = advance(current_coords, d, length);
        let border = Border {
            start: (0, 0),
            end: (0, 0),
            color,
        };
        borders.insert(border);
    }
    println!("{:?}", &borders);
    borders
}

impl Solution for Day18 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, _input: &String) -> Result<i32, String> {
        Ok(5)
    }

    fn part2(&self, _input: &String) -> Result<i32, String> {
        Ok(5)
    }
}
