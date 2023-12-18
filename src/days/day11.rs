use crate::days::common::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day11 {}

impl Day11 {
    pub fn new() -> Self {
        Day11 {}
    }
}

fn parse(
    input: &String,
) -> (
    usize,
    usize,
    HashSet<(usize, usize)>,
    Vec<usize>,
    Vec<usize>,
) {
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
    let mut empty_cols = (0..width)
        .filter(|x| !used_cols.contains(x))
        .collect::<Vec<usize>>();
    empty_cols.sort_unstable();

    (width, height, m, empty_rows, empty_cols)
}

fn l1_dist(v0: (usize, usize), v1: (usize, usize)) -> usize {
    (v1.0 as i32 - v0.0 as i32).abs() as usize + (v1.1 as i32 - v0.1 as i32).abs() as usize
}

fn expand_universe(
    planets: HashSet<(usize, usize)>,
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
    coefficient: usize,
) -> HashSet<(usize, usize)> {
    let mut expanded: HashSet<(usize, usize)> = HashSet::new();

    for &(x, y) in planets.iter() {
        let mut xx = x;
        let mut yy = y;
        for (i, &ex) in empty_cols.iter().enumerate() {
            if x > ex {
                xx += coefficient;
            }
        }
        for (i, &ey) in empty_rows.iter().enumerate() {
            if y > ey {
                yy += coefficient;
            }
        }
        expanded.insert((xx, yy));
    }

    expanded
}

fn compute_distance_sum(planets: &HashSet<(usize, usize)>) -> usize {
    let mut total: usize = 0;
    let mut already_visited: HashSet<(usize, usize)> = HashSet::new();
    for (x, y) in planets.iter() {
        for (xx, yy) in planets.iter() {
            if (xx, yy) == (x, y) || already_visited.contains(&(*xx, *yy)) {
                continue;
            }
            let d = l1_dist((*x, *y), (*xx, *yy));
            total += d;
            // println!("|{:?} - {:?}| = {:?}", (*x, *y), (*xx, *yy), d);
        }
        already_visited.insert((*x, *y));
    }
    total
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
        let expanded = expand_universe(m, &empty_rows, &empty_cols, 1);
        let total = compute_distance_sum(&expanded);

        println!("Part 1: {:?}", total);

        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        let (width, height, m, empty_rows, empty_cols) = parse(input);
        let expanded = expand_universe(m, &empty_rows, &empty_cols, 999999);
        let total = compute_distance_sum(&expanded);

        println!("Part 2: {:?}", total);
        Ok(5)
    }
}
