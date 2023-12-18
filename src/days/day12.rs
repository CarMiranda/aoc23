use crate::days::common::Solution;

pub struct Day12 {}

impl Day12 {
    pub fn new() -> Self {
        Day12 {}
    }
}

type Range = (usize, usize);

#[derive(Debug)]
struct Record {
    empty_ranges: Vec<Range>,
    filled_ranges: Vec<Range>,
    conditions: Vec<u8>,
}

impl Record {
    pub fn simplify(&mut self) {}
}

fn parse_line(line: &String) -> Record {
    let r = line
        .split(' ')
        .nth(0)
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    let conditions = line
        .split(' ')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let mut c = r[0];
    let mut s = 0;
    let mut i = 1;
    let mut empty: Vec<Range> = Vec::new();
    let mut filled: Vec<Range> = Vec::new();
    while i < r.len() {
        // Range ends
        if r[i] != c {
            match c {
                '?' => empty.push((s, i - 1)),
                '#' => filled.push((s, i - 1)),
                '.' => (),
                _ => panic!("Bananas"),
            };
            s = i;
            c = r[i];
        }
        i += 1;
    }

    match c {
        '?' => empty.push((s, i - 1)),
        '#' => filled.push((s, i - 1)),
        '.' => (),
        _ => panic!("Bananas"),
    };

    Record {
        filled_ranges: filled,
        empty_ranges: empty,
        conditions: conditions,
    }
}

fn parse(input: &String) -> Vec<Record> {
    input
        .lines()
        .map(|l| parse_line(&l.to_string()))
        .collect::<Vec<Record>>()
}

impl Solution for Day12 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        let records = parse(input);
        println!("{:?}", &records);
        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        let _records = parse(input);
        Ok(5)
    }
}
