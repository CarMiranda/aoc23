use crate::days::common::Solution;

pub struct Day15 {}

impl Day15 {
    pub fn new() -> Self {
        Day15 {}
    }
}

fn parse_word(word: &String) -> u64 {
    word
        .trim()
        .chars()
        .map(|c| c as u64)
        .fold(0, |acc, c| (acc + c) * 17 % 256)
}

fn part1(input: &String) -> u64 {
    input
        .split(",")
        .map(|w| parse_word(&w.to_string()))
        .sum::<u64>()
}

type Lens = (String, u8);

fn part2(input: &String) -> u64 {
    let mut boxes: Vec<Vec<Lens>> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    for operation in input.trim().split(",") {
        let mut label: String;
        let mut power: u8 = u8::MAX;
        let mut op: char = '-';
        if operation.ends_with('-') {
            label = operation.chars().take(operation.len() - 1).collect::<Vec<char>>().into_iter().collect();
        } else {
            label = operation.split("=").nth(0).unwrap().to_string();
            power = operation.split("=").nth(1).unwrap().parse::<u8>().unwrap();
            op = '=';
        }
        let box_id = parse_word(&label) as usize; 
        let mut bbox = boxes.get_mut(box_id).unwrap(); 
        match op {
            '=' => {
                let mut found = false;
                for (s, p) in bbox.iter_mut() {
                    if *s == *label {
                        *p = power;
                        found = true;
                        break;
                    }
                }
                if !found {
                    bbox.push((label.clone(), power));
                }
            },
            '-' => {
                let mut ii = usize::MAX;
                for (i, (s, _)) in bbox.iter().enumerate() {
                    if *s == *label {
                        ii = i;
                        break;
                    }
                }
                if ii < usize::MAX {
                    bbox.remove(ii);
                }
            },
            _ => ()
        };
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b
                .iter()
                .enumerate()
                .map(|(j, l)| (i as u64 + 1) * (j as u64 + 1) * (l.1 as u64))
                .sum::<u64>()
        })
        .sum::<u64>()
}

impl Solution for Day15 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        let r = part1(input);
        println!("{}", r);
        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        let r = part2(input);
        println!("{}", r);
        Ok(5)
    }
}
