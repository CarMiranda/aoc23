use crate::days::common::Solution;

pub struct DayNN {}

impl DayNN {
    pub fn new() -> Self {
        DayNN {}
    }
}

impl Solution for DayNN {
    type ParsedInput = String;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, input_file: String) -> Result<String, std::io::Error> {
        std::fs::read_to_string(input_file)
    }

    fn part1(&self, _input: &String) -> Result<i32, String> {
        Ok(5)
    }

    fn part2(&self, _input: &String) -> Result<i32, String> {
        Ok(5)
    }
}
