use crate::days::common::Solution;

use num::integer::lcm;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
    multi::many1,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::collections::{HashMap, HashSet};

pub struct Day08 {}

impl Day08 {
    pub fn new() -> Self {
        Day08 {}
    }
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn parse_direction(input: &str) -> IResult<&str, Vec<String>> {
    let (input, order) = many1(alt((tag("R"), tag("L"))))(input)?;
    let order = order.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    Ok((input, order))
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (rest, node) = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    )(input)?;
    let node = Node {
        id: node.0.to_string(),
        left: node.1 .0.to_string(),
        right: node.1 .1.to_string(),
    };
    Ok((rest, node))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<String>, Vec<Node>)> {
    let (input, order) = parse_direction(input)?;
    let (input, nodes) = many1(preceded(many1(newline), parse_node))(input)?;
    Ok((input, (order, nodes)))
}

impl Solution for Day08 {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, input: &String) -> Result<i32, String> {
        let (_, (order, nodes)) = parse_input(input).unwrap();
        let mut graph: HashMap<String, Node> = HashMap::new();
        for node in nodes.iter() {
            graph.insert(node.id.clone(), node.clone());
        }
        let destination = &"ZZZ".to_string();
        let mut current = graph.get(&"AAA".to_string()).unwrap();
        let mut i: usize = 0;
        let mut total: u64 = 0;
        while current.id != *destination {
            let dir = &order[i][..];
            current = match dir {
                "R" => graph.get(&current.right).unwrap(),
                "L" => graph.get(&current.left).unwrap(),
                _ => panic!("Bananas"),
            };
            i = (i + 1) % order.len();
            total += 1;
        }
        Ok(total as i32)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        let (_, (order, nodes)) = parse_input(input).unwrap();
        let mut graph: HashMap<String, Node> = HashMap::new();
        for node in nodes.iter() {
            graph.insert(node.id.clone(), node.clone());
        }

        let graph = graph;
        let currents: Vec<&Node> = graph
            .keys()
            .filter(|x| x.ends_with(&"A".to_string()))
            .map(|x| graph.get(x).unwrap())
            .collect();

        let mut r: Vec<Vec<u64>> = Vec::new();
        for &node in currents.iter() {
            r.push(identify_cycle(&graph, &order, node))
        }
        println!("{:?}", r);
        let r = vec![[12878], [14628], [16225], [18492], [19596], [21567]];
        let total: u64 = r.iter().fold(1, |acc, x| lcm(acc, x[0] as u64));
        println!("{:?}", total);
        Ok(0)
    }
}

fn has_ended(currents: &Vec<&Node>) -> bool {
    currents.iter().all(|x| x.id.ends_with(&"Z".to_string()))
}

fn identify_cycle(graph: &HashMap<String, Node>, order: &Vec<String>, start: &Node) -> Vec<u64> {
    let mut current = start;
    let mut order_position: usize = 0;
    let mut already_visited: HashSet<(&String, usize)> = HashSet::new();
    let mut total: u64 = 0;
    let mut z_positions: Vec<u64> = Vec::new();

    while !already_visited.contains(&(&current.id, order_position)) {
        already_visited.insert((&current.id, order_position));
        let dir = &order[order_position][..];
        current = match dir {
            "R" => graph.get(&current.right).unwrap(),
            "L" => graph.get(&current.left).unwrap(),
            _ => panic!("Bananas"),
        };

        order_position = (order_position + 1) % order.len();
        total += 1;
        if current.id.ends_with(&"Z".to_string()) {
            z_positions.push(total);
        }
    }

    z_positions
}
