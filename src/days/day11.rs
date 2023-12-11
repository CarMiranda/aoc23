use crate::days::common::Solution;
use std::collections::{HashSet, HashMap};

pub struct Day11 {}

impl Day11 {
    pub fn new() -> Self {
        Day11 {}
    }
}

fn parse(input: &String) -> (usize, usize, HashSet<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let mut used_cols: HashSet<usize> = HashSet::new();
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut m: HashSet<(usize, usize)> = HashSet::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    for (y, l) in input.lines().enumerate() {
        let mut is_line_empty = true;
        width = 0;
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                m.insert((x, y));
                is_line_empty = false;
                used_cols.insert(x);
            }
            width += 1;
        }
        if is_line_empty {
            empty_rows.push(y);
        }
        height += 1;
    }
    let mut empty_cols = (0..width).filter(|x| !used_cols.contains(x)).collect::<Vec<usize>>();
    empty_cols.sort_unstable();

    (width, height, m, empty_rows, empty_cols)
}

fn l1_dist(x: (usize, usize), y: (usize, usize)) {
    (y.0 - x.0) + (y.1 - x.1)
}

impl Solution for Day11 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        let (width, height, m, empty_rows, empty_cols) = parse(input);
        println!("{:?}", m);
        println!("{:?}", empty_rows);
        println!("{:?}", empty_cols);
        let mut expanded_m: HashSet<(usize, usize)> = HashSet::new();
        for x in empty_cols.iter() {
            for y in 0..height {
                if m.contains(&(x, y)) {
                    expanded_m.insert((x + 1, y));
                }
            }
        }
        for y in empty_rows.iter() {
            for x in 0..rows {
                if m.contains(&(x, y)) {
                    expanded_m.insert((x, y + 1));
                }
            }
        }

        let mut total: usize = 0;
        for (x, y) in expanded_m.iter() {
            let mut min_dist: usize = usize::MAX;
            for (xx, yy) in expanded_m.iter() {
                if (xx, yy) == (x, y) {
                    continue
                }
                let d = l1_dist((x, y), (xx, yy));
                if d < min_dist {
                    min_dist = d;
                }
            }
            total += min_dist;
        }

        println!("{:?}", total);

        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        Ok(5)
    }
}
