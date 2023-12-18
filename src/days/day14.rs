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
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                if !self.boulders.contains(&(x, y)) {
                    continue;
                }
                let mut n = self.height - 1;
                for i in ((y + 1)..self.height).rev() {
                    if self.rocks.contains(&(x, i)) || tilted.contains(&(x, i)) {
                        n = i - 1;
                    }
                }

                tilted.insert((x, n));
            }
        }
        self.boulders = tilted;
    }

    pub fn tilt_west(&mut self) {
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

    pub fn tilt_east(&mut self) {
        let mut tilted: HashSet<Coordinates> = HashSet::new();
        for x in (0..self.width).rev() {
            for y in 0..self.height {
                if !self.boulders.contains(&(x, y)) {
                    continue;
                }
                let mut n = self.width - 1;
                for i in ((x + 1)..self.width).rev() {
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

    for (y, l) in input.lines().enumerate() {
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
    println!("");
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
        platform.tilt_north();
        let r = platform.count();
        println!("{}", r);
        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        let mut platform = parse(input);
        let r = find_cycle(&mut platform, 1000000000, 50);
        println!("{}", r);
        Ok(5)
    }
}

fn find_cycle(p: &mut Platform, nb_iter: usize, buffer_size: usize) -> u64 {
    let mut i: usize = 0;
    let mut buffer: Vec<u64> = Vec::with_capacity(buffer_size);
    let mut all: Vec<u64> = Vec::new();
    let mut cycle_length = 0;
    while cycle_length == 0 {
        for i in 0..buffer_size {
            p.shake();
            buffer.push(p.count());
        }
        all.extend(buffer.clone());
        buffer.clear();

        let last_count = *all.last().unwrap();
        let mut i = 1;
        while i < all.len() {
            if let Some(previous_occurrence) =
                all.iter().rev().skip(i).position(|&x| x == last_count)
            {
                let mut li = all.len() - 1;
                let mut pi = previous_occurrence;
                while li > previous_occurrence && all[li] == all[pi] {
                    li -= 1;
                    pi -= 1;
                }
                if li == previous_occurrence {
                    cycle_length = all.len() - previous_occurrence;
                    break;
                } else {
                    i += previous_occurrence + 1;
                }
            } else {
                break;
            }
        }
    }

    let mut found: bool = true;
    let mut si: usize = 0;
    loop {
        let mut i = si;
        for i in si..(si + cycle_length) {
            if all[i] != all[i + cycle_length] {
                found = false;
                break;
            }
        }

        if found {
            break;
        }

        found = true;
        si += 1;
    }

    println!("Found cycle starting at {} with length {}", &si, &nb_iter);

    let a = (nb_iter - si) % cycle_length;
    all[a]
}
