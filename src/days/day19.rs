use nom::{
    branch::alt,
    combinator::{opt, map},
    bytes::complete::{tag, take_until},
    character::complete::{alpha0, alpha1, alphanumeric1, newline},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, tuple, separated_pair},
    IResult,
};

use crate::days::common::Solution;

use std::collections::{HashSet, HashMap};

pub struct Day19 {}

impl Day19 {
    pub fn new() -> Self {
        Day19 {}
    }
}

#[derive(Debug)]
struct Part {
    x: u8,
    m: u8,
    a: u8,
    s: u8,
}

type Rule = dyn FnOnce(Part) -> String;

fn as_rule(condition: &String, dst: &str) -> Rule {
    if dst.is_empty() {
        return |_: Part| {
            condition.clone()
        }
    }
    let mut op;
    if condition.contains("<") {
        op = '<';
    } else if condition.contains(">") {
        op = '>';
    } else {
        op = '=';
    }
    return |p: Part| {
        String::from("Banana")
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = preceded(opt(newline), alpha1)(input)?;
    let (input, rules) = delimited(
        tag("{"),
        separated_list1(
            tag(","),
            alt((   
            separated_pair(
                take_until(":"),
                tag(":"), 
                alpha1
            ),
            tuple((alpha1, alpha0))
        ))), 
        tag("}")
    )(input)?;
    Ok((input, Workflow {name: name.to_string(), rules: Vec::new()}))
}

fn parse(input: &String) {
    let workflows = input.split("\n\n").nth(0).unwrap().split("\n").map(|l| parse_workflow(l).unwrap().1).collect::<Vec<Workflow>>();
    println!("{:?}", &workflows);
}

impl Solution for Day19 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        parse(input);
        Ok(5)
    }

    fn part2(&self, _input: &String) -> Result<i32, String> {
        Ok(5)
    }
}
