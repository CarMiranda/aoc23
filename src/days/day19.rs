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

use std::{collections::{HashSet, HashMap}, hash::{Hash, Hasher}};
use std::cmp::{PartialEq, Eq};

pub struct Day19 {}

impl Day19 {
    pub fn new() -> Self {
        Day19 {}
    }
}

#[derive(Debug, Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    pub fn value(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

type Rule = dyn Fn(Part) -> String;

fn as_rule(condition: String, dst: String) -> Box<Rule> {
    // If dst is empty, it means there was no =, so condition is actually dst
    if dst.is_empty() {
        return Box::new(move |_: Part| {
            condition.clone()
        })
    }

    let lhs: char = condition.chars().nth(0).unwrap();

    if condition.contains("<") {
        let rhs: u32 = condition.split("<").nth(1).unwrap().parse::<u32>().unwrap();
        match lhs {
            'x' => return Box::new(move |p: Part| if p.x < rhs { dst.clone() } else { RULE_REJECTED.clone().to_string() }),
            'm' => return Box::new(move |p: Part| if p.m < rhs { dst.clone() } else { RULE_REJECTED.clone().to_string() }),
            'a' => return Box::new(move |p: Part| if p.a < rhs { dst.clone() } else { RULE_REJECTED.clone().to_string() }),
            's' => return Box::new(move |p: Part| if p.s < rhs { dst.clone() } else { RULE_REJECTED.clone().to_string() }),
            _ => panic!("Bananas"),
        }
    } else if condition.contains(">") {
        let rhs: u32 = condition.split(">").nth(1).unwrap().parse::<u32>().unwrap();
        match lhs {
            'x' => return Box::new(move |p: Part| if p.x > rhs { dst.clone() } else { RULE_REJECTED.clone().to_string() }),
            'm' => return Box::new(move |p: Part| if p.m > rhs { dst.clone() } else { RULE_REJECTED.clone().to_string() }),
            'a' => return Box::new(move |p: Part| if p.a > rhs { dst.clone() } else { RULE_REJECTED.clone().to_string() }),
            's' => return Box::new(move |p: Part| if p.s > rhs { dst.clone() } else { RULE_REJECTED.clone().to_string() }),
            _ => panic!("Bananas"),
        }
    } else {
        panic!("WHUT");
    }
}

struct Workflow {
    name: String,
    rules: Vec<Box<Rule>>,
}

impl Hash for Workflow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Workflow {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Workflow {}

const ACCEPTED: &str = "A";
const REJECTED: &str = "R";
const RULE_REJECTED: &str = "";

impl Workflow {
    fn apply(&self, p: Part) -> String {
        for r in self.rules.iter() {
            let rr = r(p.clone());
            match rr.as_str() {
                RULE_REJECTED => continue,
                ACCEPTED => return ACCEPTED.to_string(),
                REJECTED => return REJECTED.to_string(),
                a => return rr,
            }
        }
        panic!("BANANAS");
    }
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
            ))
        ), 
        tag("}")
    )(input)?;
    let rules = rules.iter().map(|&(c, d)| as_rule(c.to_string(), d.to_string())).collect::<Vec<Box<Rule>>>();
    Ok((input, Workflow {name: name.to_string(), rules}))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, part) = delimited(
        tag("{"),
        tuple((
            preceded(
                tag("x="),
                take_until(","),
            ),
            preceded(
                tag(",m="),
                take_until(","),
            ),
            preceded(
                tag(",a="),
                take_until(","),
            ),
            preceded(
                tag(",s="),
                take_until("}"),
            ),
        )),
        tag("}"),
    )(input)?;
    Ok((input, Part { 
        x: part.0.parse().unwrap(), 
        m: part.1.parse().unwrap(),
        a: part.2.parse().unwrap(),
        s: part.3.parse().unwrap()
    }))
}

fn parse(input: &String) {
    let workflows_vec = input.split("\n\n").nth(0).unwrap().split("\n").map(|l| parse_workflow(l).unwrap().1);
    let parts = input.split("\n\n").nth(1).unwrap().trim().split("\n").map(|l| parse_part(l).unwrap().1).collect::<Vec<Part>>();
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    for workflow in workflows_vec {
        workflows.insert(workflow.name.clone(), workflow);
    }

    let mut t: u32 = 0;
    for p in parts.iter() {
        let mut cname = "in".to_string();
        while cname != ACCEPTED && cname != REJECTED {
            let c = workflows.get(&cname).unwrap();
            cname = c.apply(p.clone());
        }
        match cname.as_str() {
            ACCEPTED => t += p.value(),
            _ => continue,
        }
    }
    println!("{}", t);
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
