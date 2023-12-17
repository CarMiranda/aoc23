use crate::days::common::Solution;

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

const Directions: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

pub struct Day17 {}

impl Day17 {
    pub fn new() -> Self {
        Day17 {}
    }
}

fn parse(input: &String) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
        .collect()
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Node {
    heat_loss: u32,
    dir: (isize, isize),
    coords: (usize, usize),
    straight: u8,
}

type CheckFn = fn((isize, isize), (isize, isize), u8) -> u8;

fn part1_check(dir0: (isize, isize), dir1: (isize, isize), straight: u8) -> u8 {
    if dir1 == (-dir0.0, -dir0.1) {
        return 0
    }
    if dir1 == dir0 {
        return straight + 1
    }
    if dir1 != dir0 {
        return 1
    }
    0
}

fn part2_check(dir0: (isize, isize), dir1: (isize, isize), straight: u8) -> u8 {
    if dir1 == (-dir0.0, -dir0.1) {
        return 0
    }
    if dir1 == dir0 {
        return straight + 1
    }
    if straight >= 4 || dir0 == (0, 0) {
        return 1
    }
    0
}

fn dijkstra(m: &Vec<Vec<u8>>, src: (usize, usize), check_fn: CheckFn, max_straight: u8) -> HashMap<(usize, usize), HashMap<(isize, isize, u8), u32>> {
    let mut front: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    let src_node = Node {heat_loss: 0, dir: (0, 0), coords: src, straight: 1};
    front.push(Reverse(src_node));

    let mut distances: HashMap<(usize, usize), HashMap<(isize, isize, u8), u32>> = HashMap::new();
    for y in 0..m.len() {
        for x in 0..m[0].len() {
            distances.insert((x, y), HashMap::new());
        }
    }

    while !front.is_empty() {
        //println!("{:?}", &front);
        let current = front.pop().unwrap().0;
        for &(dx, dy) in Directions.iter() {
            let sn = check_fn(current.dir, (dx, dy), current.straight);
            if sn == 0 || sn == max_straight {
                continue;
            }

            let (xn, yn) = (current.coords.0.checked_add_signed(dx), current.coords.1.checked_add_signed(dy));
            if yn.is_none() || xn.is_none() {
                continue;
            }

            let (xn, yn) = (xn.unwrap(), yn.unwrap());
            if yn < m.len() && xn < m[0].len() {
                let hn: u32 = current.heat_loss + m[yn][xn] as u32;
                let d = distances
                    .get_mut(&(xn, yn))
                    .unwrap();

                if hn < *d.get(&(dx, dy, sn)).unwrap_or(&u32::MAX) {
                    if d.contains_key(&(dx, dy, sn)) {
                        *d.get_mut(&(dx, dy, sn)).unwrap() = hn;
                    } else {
                        d.insert((dx, dy, sn), hn);
                    }
                    let new_node = Node {
                        heat_loss: hn,
                        coords: (xn, yn),
                        dir: (dx, dy),
                        straight: sn
                    };
                    front.push(Reverse(new_node));
                }
            }
        }
    }
    distances
}

impl Solution for Day17 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        let m = parse(input);
        let distances = dijkstra(&m, (0, 0), part1_check, 3);
        let r = distances
            .get(&(m[0].len()-1, m.len()-1))
            .unwrap()
            .values()
            .min()
            .unwrap();
        println!("{}", r);
        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        let m = parse(input);
        let distances = dijkstra(&m, (0, 0), part2_check, 11);
        let r = distances
            .get(&(m[0].len()-1, m.len()-1))
            .unwrap()
            .iter()
            .filter_map(|((_, _, s), h)| if *s >= 4 { Some(h) } else { None })
            .min()
            .unwrap();
        println!("{}", r);
        Ok(5)
    }
}
