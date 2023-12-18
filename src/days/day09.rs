use crate::days::common::Solution;

pub struct Day09 {}

impl Day09 {
    pub fn new() -> Self {
        Day09 {}
    }
}

fn parse(input: &String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn solve_history(history: &Vec<i32>) -> i32 {
    let mut diffs: Vec<i32> = Vec::new();
    let mut all_zeros: bool = true;
    for i in 0..(history.len() - 1) {
        let d = history[i + 1] - history[i];
        diffs.push(d);
        if all_zeros && d != 0 {
            all_zeros = false;
        }
    }
    let l = history[history.len() - 1];
    if all_zeros {
        return l;
    }
    return l + solve_history(&diffs);
}

fn solve_history_bw(history: &Vec<i32>) -> i32 {
    let mut diffs: Vec<i32> = Vec::new();
    let mut all_zeros: bool = true;
    for i in 0..(history.len() - 1) {
        let d = history[i + 1] - history[i];
        diffs.push(d);
        if all_zeros && d != 0 {
            all_zeros = false;
        }
    }
    let l = history[0];
    if all_zeros {
        return l;
    }
    return l - solve_history_bw(&diffs);
}

impl Solution for Day09 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        let histories = parse(input);
        let r: i32 = histories.iter().map(|h| solve_history(h)).sum();
        Ok(r)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        let histories = parse(input);
        let r: i32 = histories.iter().map(|h| solve_history_bw(h)).sum();
        Ok(r)
    }
}
