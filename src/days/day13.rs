use crate::days::common::Solution;

pub struct Day13 {}

impl Day13 {
    pub fn new() -> Self {
        Day13 {}
    }
}

#[derive(Debug)]
struct Pattern {
    mirrors: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

fn parse_pattern(input: &String) -> Pattern {
    let mirrors = input
        .lines()
        .map(|l| {
            l
                .chars()
                .map(|c| c == '#')
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();
    Pattern { 
        width: mirrors[0].len(), 
        height: mirrors.len(),
        mirrors: mirrors,
    }
}

fn parse(input: &String) -> u64 {
    input
        .split("\n\n")
        .map(|p| {
            let pattern = parse_pattern(&p.to_string());
            find_symmetries(&pattern) as u64
        })
        .inspect(|x| println!("{}", x))
        .sum::<u64>()
}

fn find_symmetry(v: &Vec<bool>) -> Vec<usize> {
    let mut axes: Vec<usize> = Vec::new();
    for i in 1..(v.len()-1) {
        if check_symmetry(v, i) {
            axes.push(i);
        }
    }
    axes
}

fn col_major(m: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut t: Vec<Vec<bool>> = Vec::new();
    for j in 0..m[0].len() {
        let mut mirrors_col: Vec<bool> = Vec::new();
        for i in 0..m.len() {
            mirrors_col.push(m[i][j]);
        }
        t.push(mirrors_col);
    }
    t
}



fn check_symmetry(v: &Vec<bool>, i: usize) -> bool {
    let mut s = i - 1;
    let mut e = i;
    while e < v.len() && v[s] == v[e] {
        if s == 0 {
            break;
        }
        s -= 1;
        e += 1;
    }
    e == v.len() || (s == 0 && v[s] == v[e])
}

fn check_symmetries(v: &Vec<bool>, s: &Vec<usize>) -> Vec<usize> {
    let mut f: Vec<usize> = Vec::new();
    for ss in s.iter() {
        if check_symmetry(v, *ss) {
            f.push(*ss);
        }
    }
    f
}

fn find_row_symmetry(p: &Pattern) -> u64 {
    let mut row_symmetry: Vec<usize> = find_symmetry(&p.mirrors[0]);
    //println!("Found symmetry at {:?}", row_symmetry);
    if row_symmetry.len() > 0 {
        let mut i = 1;
        while i < p.height && row_symmetry.len() > 0 {
            row_symmetry = check_symmetries(&p.mirrors[i], &row_symmetry);
            i += 1;
        }
        if row_symmetry.len() > 0 {
            return row_symmetry[0] as u64;
        }
    }
    0
}

fn print_board(m: &Vec<Vec<bool>>) {
    for r in m.iter() {
        for c in r.iter() {
            print!("{}", if *c { "#" } else { "." });
        }
        println!();
    }
}

fn find_symmetries(p: &Pattern) -> u64 {

    let row_symmetry = find_row_symmetry(p);
    //print_board(&p.mirrors);
    let mirrors = col_major(&p.mirrors);
    let p = Pattern {
        width: p.height,
        height: p.width,
        mirrors: mirrors,
    };
    let col_symmetry = find_row_symmetry(&p);
    row_symmetry + 100 * col_symmetry
}

impl Solution for Day13 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        // let patterns = parse(input);
        // let mut total = 0u64; 
        // for pattern in patterns.iter() {
        //     total += find_symmetries(&pattern);
        // }
        let total = parse(input);
        println!("{}", &total);
        Ok(5)
    }

    fn part2(&self, _input: &String) -> Result<i32, String> {
        Ok(5)
    }
}
