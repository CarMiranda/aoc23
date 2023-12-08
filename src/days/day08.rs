use crate::days::common::Solution;
use nom::{
    branch::alt,
    combinator::opt,
    bytes::complete::tag,
    multi::many1,
    character::complete::{
        alpha1,
        space1,
        newline
    },
    sequence::{delimited, separated_pair, tuple, preceded},
    IResult
};

pub struct Day08 {}

impl Day08 {
    pub fn new() -> Self {
        Day08 {}
    }
}

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn parse_direction(input: &str) -> IResult<&str, Vec<&str>> {
    many1(alt((tag("R"), tag("L"))))(input)
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (rest, node) = separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(
                alpha1,
                tag(", "),
                alpha1
            ),
            tag(")")
        )
    )(input)?;
    let node = Node {
        id: node.0.to_string(),
        left: node.1.0.to_string(),
        right: node.1.1.to_string()
    };
    Ok((rest, node))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<Node>)> {
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
        let (order, nodes) = parse_input(input).unwrap();
        println!("Order {}", &order);
        println!("Nodes {:?}", &nodes);
        Ok(5)
    }

    fn part2(&self, input: &String) -> Result<i32, String> {
        Ok(5)
    }
}
