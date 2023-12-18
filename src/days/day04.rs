use crate::days::common::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day04 {}

impl Day04 {
    pub fn new() -> Self {
        Day04 {}
    }
}

impl Solution for Day04 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        let r = input
            .trim_end()
            .split("\n")
            .map(|l| l.split(": ").nth(1).unwrap().trim_end().replace("  ", " "))
            .map(|l| {
                let w = l
                    .split(" | ")
                    .nth(0)
                    .unwrap()
                    .split(" ")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<HashSet<i32>>();
                let m = l
                    .split(" | ")
                    .nth(1)
                    .unwrap()
                    .split(" ")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<HashSet<i32>>();
                let inter = w.intersection(&m);
                inter.fold(1, |a, x| 2 * a) / 2
            })
            .sum::<i32>();
        Ok(r)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        let mut hm = HashMap::<i32, usize>::new();
        let r = input
            .trim_end()
            .split("\n")
            .map(|l| {
                let card = l
                    .split(": ")
                    .nth(0)
                    .unwrap()
                    .split(" ")
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                let nums = l.split(": ").nth(1).unwrap().trim_end().replace("  ", " ");
                (card, nums)
            })
            .map(|(card, nums)| {
                let w = nums
                    .split(" | ")
                    .nth(0)
                    .unwrap()
                    .split(" ")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<HashSet<i32>>();
                let m = nums
                    .split(" | ")
                    .nth(1)
                    .unwrap()
                    .split(" ")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<HashSet<i32>>();
                let inter = w.intersection(&m).copied().collect::<Vec<i32>>();
                (card, inter)
            });

        for (c, inter) in r {
            let mut multiplier = 1;
            if hm.contains_key(&c) {
                multiplier = hm[&c];
            } else {
                hm.insert(c, 1);
            }
            let n2xc = inter.len();
            for i in 1..=n2xc {
                let nc = c + i as i32;
                if hm.contains_key(&nc) {
                    *hm.get_mut(&nc).unwrap() += multiplier;
                } else {
                    hm.insert(nc, 1 + multiplier);
                }
            }
        }

        //2u32.pow(*x as u32 - 1)
        let r = hm
            .iter()
            .map(|(c, x)| {
                println!("{} {}", c, x);
                *x as i32
            })
            .sum::<i32>();
        println!("{}", r);

        Ok(r as i32)
    }
}
