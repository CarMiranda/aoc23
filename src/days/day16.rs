use crate::days::common::Solution;

use std::collections::{HashMap, HashSet};

pub struct Day16 {}

impl Day16 {
    pub fn new() -> Self {
        Day16 {}
    }
}

fn parse(input: &String) -> (HashMap<(usize, usize), char>, usize, usize) {
    let mut m: HashMap<(usize, usize), char> = HashMap::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                m.insert((x, y), c);
            }
            width = x;
        }
        height = y;
    }
    (m, width + 1, height + 1)
}

fn advance(c: (usize, usize), d: u8) -> (usize, usize) {
    match d {
        0 => (c.0 + 1, c.1),
        1 => (c.0, c.1.wrapping_sub(1)),
        2 => (c.0.wrapping_sub(1), c.1),
        3 => (c.0, c.1 + 1),
        _ => panic!("Bananas"),
    }
}

fn print_visited(v: &HashSet<(usize, usize)>, w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            if v.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn bounce(c: char, d: u8) -> u8 {
    match (c, d) {
        ('/', 0) => 1,
        ('/', 1) => 0,
        ('/', 2) => 3,
        ('/', 3) => 2,
        ('\\', 0) => 3,
        ('\\', 1) => 2,
        ('\\', 2) => 1,
        ('\\', 3) => 0,
        ('|', 0) | ('|', 2) => 4,
        ('-', 0) | ('|', 1) | ('-', 2) | ('|', 3) => d,
        ('-', 1) | ('-', 3) => 5,
        _ => panic!("BBananas"),
    }
}

fn part1(
    map: &HashMap<(usize, usize), char>,
    width: usize,
    height: usize,
    init: (usize, usize, u8),
) -> u32 {
    let mut front: Vec<(usize, usize, u8)> = Vec::new();
    let mut visited: HashSet<(usize, usize, u8)> = HashSet::new();

    if let Some(&c) = map.get(&(init.0, init.1)) {
        let init_dir = bounce(c, init.2);
        match init_dir {
            0..=3 => front.push((init.0, init.1, init_dir)),
            4 => {
                front.push((init.0, init.1, 1));
                front.push((init.0, init.1, 3));
            }
            5 => {
                front.push((init.0, init.1, 0));
                front.push((init.0, init.1, 2));
            }
            _ => panic!("BBBananas"),
        };
    } else {
        front.push(init);
    }

    for p in front.iter() {
        visited.insert(p.clone());
    }
    // 0 = right, 1 = up, 2 = left, 3 = down

    while front.len() > 0 {
        let mut new_front: Vec<(usize, usize, u8)> = Vec::new();
        for &(x, y, d) in front.iter() {
            if x >= width || y >= height || (x == 0 && d == 2) || (y == 0 && d == 1) {
                continue;
            }
            let (xn, yn) = advance((x, y), d);

            if xn >= width || yn >= height || visited.contains(&(xn, yn, d)) {
                continue;
            } else {
                visited.insert((xn, yn, d));
            }

            if let Some(&c) = map.get(&(xn, yn)) {
                let dn = bounce(c, d);
                match dn {
                    0..=3 => new_front.push((xn, yn, dn)),
                    4 => {
                        new_front.push((xn, yn, 1));
                        new_front.push((xn, yn, 3));
                    }
                    5 => {
                        new_front.push((xn, yn, 0));
                        new_front.push((xn, yn, 2));
                    }
                    _ => panic!("BBBananas"),
                };
            } else {
                new_front.push((xn, yn, d));
            }
        }
        front = new_front.clone();
    }

    //println!("{}", visited.len());
    let visited: HashSet<(usize, usize)> = visited.iter().map(|&(x, y, _)| (x, y)).collect();
    //print_visited(&visited, width, height);
    visited.len() as u32
}

impl Solution for Day16 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        let (m, w, h) = parse(input);
        let mm = part1(&m, w, h, (0, 0, 0));
        println!("{}", mm);
        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        let (m, w, h) = parse(input);
        let mut mm: u32 = 0;
        for i in 0..w {
            let m0 = part1(&m, w, h, (i, 0, 3));
            let m1 = part1(&m, w, h, (i, h - 1, 1));
            let m2 = part1(&m, w, h, (0, i, 0));
            let m3 = part1(&m, w, h, (w - 1, i, 2));
            mm = *vec![m0, m1, m2, m3, mm].iter().max().unwrap();
        }
        println!("{}", &mm);
        Ok(5)
    }
}
