use crate::days::common::Solution;

pub struct Day05 {}

impl Day05 {
    pub fn new() -> Self {
        Day05 {}
    }
}

fn apply_map(seed: u64, m: &Vec<(u64, u64, u64)>) -> u64 {
    for &(dst, src, n) in m.iter() {
        if seed < src + n && seed >= src {
            return seed - src + dst;
        }
    }
    return seed;
}

fn apply_maps(seed: u64, maps: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut r = seed;
    for map in maps.iter() {
        r = apply_map(r, map);
    }
    r
}

fn parse(input: &String) -> () {
    let seeds = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let maps: Vec<Vec<(u64, u64, u64)>> = input
        .split("\n\n")
        .skip(1)
        .map(|x| {
            let x: Vec<(u64, u64, u64)> = x
                .split("\n")
                .skip(1)
                .map(|y| {
                    let y = y
                        .split(" ")
                        .map(|z| z.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    (y[0], y[1], y[2])
                })
                .collect();
            x
        })
        .collect();

    println!("{:?}", maps);
    let mut r: Vec<u64> = Vec::new();
    for &seed in seeds.iter() {
        r.push(apply_maps(seed, &maps));
    }
    println!("{:?}", r.iter().min());

    let mut seed_ranges: Vec<(u64, u64)> = Vec::new();
    for i in (0..(seeds.len() - 1)).step_by(2) {
        seed_ranges.push((seeds[i], seeds[i + 1]));
    }
    let mut r = u64::MAX;
    for &(s, n) in seed_ranges.iter() {
        for i in 0..n {
            let rr = apply_maps(s + i as u64, &maps);
            if rr < r {
                r = rr;
                println!("{}", r);
            }
        }
    }
}

impl Solution for Day05 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        parse(&input.trim_end().to_string());
        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        Ok(5)
    }
}
