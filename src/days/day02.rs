use crate::days::common::Solution;
use nom::{
    branch::alt,
    combinator::opt,
    bytes::complete::tag,
    multi::many1,
    character::complete::{
        u8 as nomu8,
        i32 as nomi32,
        space1
    },
    sequence::{tuple, preceded},
    IResult
};

pub struct Day02 {}

impl Day02 {
    pub fn new() -> Self {
        Day02 {}
    }
}

#[derive(Debug)]
struct Bag(u32, u32, u32);

impl Bag {
    pub fn contains(&self, other: &Bag) -> bool {
        self.0 >= other.0 && self.1 >= other.1 && self.2 >= other.2
    }

    pub fn merge(self, other: Bag) -> Bag {
        Bag(
            if self.0 > other.0 { self.0 } else { other.0 },
            if self.1 > other.1 { self.1 } else { other.1 },
            if self.2 > other.2 { self.2 } else { other.2 },
        )
    }

    pub fn prod(&self) -> u32 {
        self.0 * self.1 * self.2
    }
}

impl Into<Bag> for &Vec<(u8, &str)> {
    fn into(self) -> Bag {
        self.iter().fold(Bag(0, 0, 0), |a, (i, c)| {
            match c {
                &"red" => Bag(*i as u32, a.1, a.2),
                &"green" => Bag(a.0, *i as u32, a.2),
                &"blue" => Bag(a.0, a.1, *i as u32),
                &_ => panic!("This should not happen"),
            }
        })
    }
}

fn parse_sample(input: &str) -> IResult<&str, Vec<(u8, &str)>> {
    many1(
        preceded(
            opt(tag(", ")),
            tuple((
                nomu8,
                preceded(
                    space1,
                    alt((
                        tag("red"),
                        tag("green"),
                        tag("blue")
                    )),
                ),
            ))
        )
    )(input)
}

fn parse_samples(input: &str) -> IResult<&str, Vec<Vec<(u8, &str)>>> {
    many1(
        preceded(
            opt(tag("; ")),
            parse_sample
        )
    )(input)
}

fn parse_game(input: &str) -> IResult<&str, (i32, Vec<Vec<(u8, &str)>>)> {
    tuple((
        preceded(
            tag("Game "),
            nomi32,
        ),
        preceded(
            tag(": "),
            parse_samples
        ),
    ))(input)
}


fn parse_bag(s: &String) -> Bag {
    let res = Bag(0, 0, 0);
    s.split(", ").fold(res, |mut r, ball| {
        if ball.ends_with("d") {
            r.0 = ball.split(" ").nth(0).unwrap().trim().parse::<u32>().unwrap();
        } else if ball.ends_with("n") {
            r.1 = ball.split(" ").nth(0).unwrap().trim().parse::<u32>().unwrap();
        } else if ball.ends_with("e") {
            r.2 = ball.split(" ").nth(0).unwrap().trim().parse::<u32>().unwrap();
        }
        r
    })
}

fn parse_line(line: &String) -> (u32, Bag) {
    let game = line.split(":").nth(0).unwrap().split(" ").nth(1).unwrap().parse::<u32>().unwrap();
    let bag = line
        .split(": ")
        .nth(1)
        .unwrap()
        .split("; ")
        .map(|b| parse_bag(&b.to_string()))
        .fold(Bag(0, 0, 0), |pb, nb| {
            pb.merge(nb)
        });
    (game, bag)
}

fn part1_alt(input: &String) -> Result<i32, String> {
    let config = Bag(12, 13, 14);
    let r = input
        .trim_end()
        .split("\n")
        .map(|g| {
            let (_, (i, v)) = parse_game(g).unwrap();
            let v = v.iter().fold(
                Bag(0, 0, 0), 
                |a, b| {
                    let b: Bag = b.into();
                    a.merge(b)
                }
            );
            (i, v)
        })
        .filter(|(_, b)| config.contains(b))
        .map(|(i, _)| i as i32)
        .sum::<i32>();
    Ok(r)
}

fn part2_alt(input: &String) -> Result<i32, String> {
    let r = input
        .trim_end()
        .split("\n")
        .map(|g| {
            let (_, (_, v)) = parse_game(g).unwrap();
            let v = v.iter().fold(
                Bag(0, 0, 0), 
                |a, b| {
                    let b: Bag = b.into();
                    a.merge(b)
                }
            );
            v.prod()
        })
        .sum::<u32>();
    Ok(r as i32)
}

fn part1(input: &String) -> Result<i32, String> {
    let config = Bag(12, 13, 14);
    let r = input
        .trim_end()
        .split("\n")
        .map(|line| parse_line(&line.to_string()))
        .filter(|(_, b)| config.contains(b))
        .map(|(i, _)| i)
        .sum::<u32>();
    Ok(r as i32)
}

fn part2(input: &String) -> Result<i32, String> {
    let r = input
        .trim_end()
        .split("\n")
        .map(|line| {
            let (_, b) = parse_line(&line.to_string());
            b.prod()
        })
        .sum::<u32>();
    Ok(r as i32)
}

impl Solution for Day02 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }


    fn part1(&self, input: &String) -> Result<i32, String> {
        part1_alt(input)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        part2_alt(input)
    }
}

