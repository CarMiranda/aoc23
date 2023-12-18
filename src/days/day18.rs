

use crate::days::common::Solution;

pub struct Day18 {}

impl Day18 {
    pub fn new() -> Self {
        Day18 {}
    }
}

fn parse(input: &String) -> Vec<(u8, usize)> {
    let mut v: Vec<(u8, usize)> = Vec::new();
    for l in input.lines() {
        let d = match l.split(" ").nth(0).unwrap() {
            "R" => 0,
            "U" => 1,
            "L" => 2,
            "D" => 3,
            _ => panic!("Bananas"),
        };
        let length = l.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
        v.push((d, length));
    }
    v
}

fn parse2(input: &String) -> Vec<(u8, usize)> {
    let mut v: Vec<(u8, usize)> = Vec::new();
    for l in input.lines() {
        let color = l
            .split(" ")
            .nth(2)
            .unwrap()
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .to_string();
        let d = match color.chars().nth(5).unwrap() {
            '0' => 0,
            '1' => 3,
            '2' => 2,
            '3' => 1,
            _ => panic!("Bananas"),
        };
        let depth =
            usize::from_str_radix(color.chars().take(5).collect::<String>().as_str(), 16).unwrap();
        v.push((d, depth));
    }
    v
}

fn dmap(d: u8) -> (isize, isize) {
    match d {
        0 => (1, 0),
        1 => (0, -1),
        2 => (-1, 0),
        3 => (0, 1),
        _ => panic!("BBananas"),
    }
}

fn poly_area(poly: &Vec<(u8, usize)>) -> usize {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut perimeter: usize = 0;
    let mut area: isize = 0;
    for &(d, length) in poly.iter() {
        let (dx, dy) = dmap(d);
        let (dx, dy) = (dx * length as isize, dy * length as isize);
        x = x + dx;
        y = y + dy;
        perimeter += length;
        area += x * dy;
    }
    area as usize + perimeter / 2 + 1
}

impl Solution for Day18 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        let v = parse(input);
        let r = poly_area(&v);
        println!("{}", r);
        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        let v = parse2(input);
        let r = poly_area(&v);
        println!("{}", r);
        Ok(5)
    }
}
