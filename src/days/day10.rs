use crate::days::common::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day10 {}

impl Day10 {
    pub fn new() -> Self {
        Day10 {}
    }
}

fn replace_s(p: (usize, usize), m: &HashMap<(usize, usize), char>) -> char {
    let mut nmap = (false, false, false, false);
    // Up
    if p.1 > 0 {
        if let Some(c) = m.get(&(p.0, p.1 - 1)) {
            match c {
                'F' | '7' | '|' => nmap.0 = true,
                _ => (),
            };
        };
    }
    // Down
    if let Some(c) = m.get(&(p.0, p.1 + 1)) {
        match c {
            'J' | 'L' | '|' => nmap.1 = true,
            _ => (),
        };
    };
    // right
    if let Some(c) = m.get(&(p.0 + 1, p.1)) {
        match c {
            'J' | '7' | '-' => nmap.2 = true,
            _ => (),
        };
    };
    // left
    if p.0 > 0 {
        if let Some(c) = m.get(&(p.0 - 1, p.1)) {
            match c {
                'F' | 'L' | '-' => nmap.3 = true,
                _ => (),
            };
        };
    }

    match nmap {
        (true, true, false, false) => '|',
        (true, false, true, false) => 'L',
        (true, false, false, true) => 'J',
        (false, true, true, false) => 'F',
        (false, true, false, true) => '7',
        (false, false, true, true) => '-',
        _ => panic!(),
    }
}

fn connected_neighbours(c: char, p: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    match c {
        '-' => ((p.0 - 1, p.1), (p.0 + 1, p.1)),
        'J' => ((p.0, p.1 - 1), (p.0 - 1, p.1)),
        '|' => ((p.0, p.1 - 1), (p.0, p.1 + 1)),
        'L' => ((p.0, p.1 - 1), (p.0 + 1, p.1)),
        'F' => ((p.0, p.1 + 1), (p.0 + 1, p.1)),
        '7' => ((p.0, p.1 + 1), (p.0 - 1, p.1)),
        'S' => (p, p),
        _ => panic!(),
    }
}

fn parse(input: &String) -> ((usize, usize), HashMap<(usize, usize), char>) {
    let mut s: (usize, usize) = (0, 0);
    let mut m: HashMap<(usize, usize), char> = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                m.insert((x, y), c);
                if c == 'S' {
                    s = (x, y);
                }
            }
        }
    }
    (s, m)
}

fn sort_polygon(polygon: &HashSet<(usize, usize)>) -> HashMap<usize, Vec<usize>> {
    let mut m: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(x, y) in polygon.iter() {
        if let Some(ref mut xv) = m.get_mut(&x) {
            xv.push(y);
        } else {
            m.insert(x, vec![y]);
        }
    }
    for col in m.values_mut() {
        col.sort_unstable()
    }
    m
}

fn merge_segments(v: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut i: usize = 0;
    let mut r: Vec<(usize, usize)> = Vec::new();
    while i < v.len() {
        let mut j = i;
        while j < v.len() - 1 && v[j + 1] == v[j] + 1 {
            j += 1;
        }
        if i != j {
            r.push((v[i], v[j]));
        }
        if i != 0 && i == j {
            r.push((v[i], v[j]));
        }
        i = j + 1;
    }
    r
}

fn get_column_area(column: &Vec<usize>) -> u64 {
    let mut isin = false;
    let mut area: u64 = 0;
    let column = merge_segments(column);
    let mut i: usize = 0;
    let mut ybase: u64 = 0;

    for &(ys, ye) in column.iter() {
        if ye == ys {
            if isin {
                area += ys as u64 - ybase - 1;
            }
            isin = !isin;
            ybase = ys as u64;
        } else {
            if isin {
                area += ys as u64 - ybase - 1;
                ybase = ye as u64;
            }
        }
    }
    area
}

fn get_inner_area(polygon: &HashSet<(usize, usize)>) -> u64 {
    let polygon = sort_polygon(polygon);
    let mut total: u64 = 0;
    for (_, v) in polygon.iter() {
        total += get_column_area(v);
    }
    total
}

fn determinant(v: &Vec<(usize, usize)>) -> i32 {
    v.iter()
        .fold((0, v.last().unwrap()), |(d, p), n| {
            let dd = (p.0 * n.1) as i32 - (p.1 * n.0) as i32;
            (d + dd, n)
        })
        .0
}

fn get_inners_area(polygon: &Vec<(usize, usize)>) -> u64 {
    let mut total: u64 = 0;
    let det = determinant(&polygon).abs() / 2;
    println!("{:?}", det);
    println!("{:?}", det - polygon.len() as i32);
    6
}

impl Solution for Day10 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        let (sc, m) = parse(input);
        let s = replace_s(sc, &m);
        let (_, mut n) = connected_neighbours(s, sc);
        let mut c = m.get(&n).unwrap();
        let mut prev = sc;

        let mut loop_size: u64 = 0;
        let mut lo: HashSet<(usize, usize)> = HashSet::new();
        let mut vo: Vec<(usize, usize)> = Vec::new();

        lo.insert(n.clone());
        vo.push(n.clone());
        loop {
            let (n1, n2) = connected_neighbours(*c, n);
            // println!("Neigh {:?} {:?}", n1, n2);
            // println!("prev={:?}", prev);
            if n1 == prev {
                prev = n;
                n = n2;
            } else {
                prev = n;
                n = n1;
            }
            c = m.get(&n).unwrap();
            lo.insert(n.clone());
            vo.push(n.clone());
            // println!("n={:?} c={:?}", n, c);
            // println!("prev={:?}", prev);
            loop_size += 1;
            if *c == 'S' {
                break;
            }
        }
        println!("{}", loop_size / 2 + 1);

        // part 2
        let mut mm: HashMap<(usize, usize), char> = HashMap::new();
        for (k, v) in m.iter() {
            if lo.contains(k) {
                mm.insert(*k, *v);
            }
        }
        let mut m = mm;
        m.insert(sc, s);

        let mut in_cnt = 0;
        for y in 0..140 {
            let mut isin = false;
            for x in 0..140 {
                if let Some(c) = m.get(&(x, y)) {
                    match c {
                        '|' | 'F' | '7' => isin = !isin,
                        _ => (),
                    }
                } else if isin {
                    in_cnt += 1;
                }
            }
        }
        println!("{}", in_cnt);
        Ok(in_cnt)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        Ok(5)
    }
}
