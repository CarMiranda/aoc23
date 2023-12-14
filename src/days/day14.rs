use crate::days::common::Solution;

use std::collections::HashSet;

pub struct Day14 {}

impl Day14 {
    pub fn new() -> Self {
        Day14 {}
    }
}


type Coordinates = (usize, usize);


#[derive(Debug)]
struct Platform {
    boulders: HashSet<(usize, usize)>,
    rocks: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Platform {
    pub fn tilt_north(&mut self) {
        let mut tilted: HashSet<Coordinates> = HashSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.boulders.contains(&(x, y)) {
                    continue;
                }
                let mut n = 0;
                for i in 0..y {
                    if self.rocks.contains(&(x, i)) || tilted.contains(&(x, i)) {
                        n = i + 1;
                    }
                }

                tilted.insert((x, n));
            }
        }
        self.boulders = tilted;
    }

    pub fn tilt_south(&mut self) {
        let mut tilted: HashSet<Coordinates> = HashSet::new();
        for y in (self.height-1)..=0 {
            for x in 0..self.width {
                if !self.boulders.contains(&(x, y)) {
                    continue;
                }
                let mut n = 0;
                for i in 0..y {
                    if self.rocks.contains(&(x, i)) || tilted.contains(&(x, i)) {
                        n = i - 1;
                    }
                }

                tilted.insert((x, n));
            }
        }
        self.boulders = tilted;
    }

    pub fn tilt_east(&mut self) {
        let mut tilted: HashSet<Coordinates> = HashSet::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if !self.boulders.contains(&(x, y)) {
                    continue;
                }
                let mut n = 0;
                for i in 0..x {
                    if self.rocks.contains(&(i, y)) || tilted.contains(&(i, y)) {
                        n = i + 1;
                    }
                }

                tilted.insert((n, y));
            }
        }
        self.boulders = tilted;
    }

    pub fn tilt_west(&mut self) {
        let mut tilted: HashSet<Coordinates> = HashSet::new();
        for x in (self.width-1)..=0 {
            for y in 0..self.height {
                if !self.boulders.contains(&(x, y)) {
                    continue;
                }
                let mut n = 0;
                for i in 0..x {
                    if self.rocks.contains(&(i, y)) || tilted.contains(&(i, y)) {
                        n = i - 1;
                    }
                }

                tilted.insert((n, y));
            }
        }
        self.boulders = tilted;
    }

    pub fn shake(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn count(&self) -> u64 {
        let mut total: u64 = 0;
        for y in 0..self.height {
            let mut n: u64 = 0;
            for x in 0..self.width {
                if self.boulders.contains(&(x, y)) {
                    n += 1;
                }
            }
            total += n * (self.height - y) as u64;
        }
        total
    }
}

fn parse(input: &String) -> Platform {
    let mut rocks: HashSet<Coordinates> = HashSet::new();
    let mut boulders: HashSet<Coordinates> = HashSet::new();
    let mut width = 0;
    let mut height = 0;

    for (y, l) in  input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => rocks.insert((x, y)),
                'O' => boulders.insert((x, y)),
                _ => false,
            };
            width = x;
        }
        height = y;
    }
    width += 1;
    height += 1;
    Platform {
        boulders,
        rocks,
        width,
        height,
    }
}

fn print_platform(p: &Platform) {
    for y in 0..p.height {
        for x in 0..p.width {
            if let Some(c) = p.boulders.get(&(x, y)) {
                print!("O");
            } else if let Some(c) = p.rocks.get(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

impl Solution for Day14 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        let mut platform = parse(input);
        //print_platform(&platform);
        platform.tilt_north();
        //println!("");
        //print_platform(&platform);
        let r = platform.count();
        println!("{}", r);
        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        let mut platform = parse(input);
        //print_platform(&platform);
        for _ in 0..1000000000 {
            platform.shake();
        }
        //println!("");
        //print_platform(&platform);
        let r = platform.count();
        println!("{}", r);
        Ok(5)
    }
}
