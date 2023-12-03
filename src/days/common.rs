use std::fmt::Display;

pub trait Solution {
    type ParsedInput: Display;
    type Part1Output: Display;
    type Part2Output: Display;

    fn parse(&self, input_file: String) -> Result<Self::ParsedInput, std::io::Error>;
    fn part1(&self, input: &Self::ParsedInput) -> Result<Self::Part1Output, String>;
    fn part2(&self, input: &Self::ParsedInput) -> Result<Self::Part2Output, String>;

    fn run(&self, input_file: String, part: Option<u8>) -> Result<(Result<Self::Part1Output, String>, Result<Self::Part2Output, String>), String> {
        match self.parse(input_file) {
            Err(e) => Err(e.to_string()),
            Ok(c) => {
                match part {
                    Some(1u8) => {
                        let r1 = self.part1(&c);
                        return Ok((r1, Err("Not requested.".into())));
                    },
                    Some(2u8) => {
                        let r2 = self.part2(&c);
                        return Ok((Err("Not requested.".into()), r2));
                    },
                    None => {
                        let r1 = self.part1(&c);
                        let r2 = self.part2(&c);
                        return Ok((r1, r2));
                    },
                    Some(_) => {
                        return Err("Unexpected part identifier.".into())
                    }
                }
            }
        }
    }
}


