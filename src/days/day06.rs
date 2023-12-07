use crate::days::common::Solution;

pub struct Day06 {}

impl Day06 {
    pub fn new() -> Self {
        Day06 {}
    }
}

fn parse(input: &String) -> () {
   let times: Vec<u64> = input
       .lines()
       .nth(0).unwrap()
       .split(":")
       .nth(1).unwrap()
       .split(" ")
       .filter(|x| !x.is_empty())
       .map(|x| x.parse::<u64>().unwrap())
       .collect(); 
   let records: Vec<u64> = input
       .lines()
       .nth(1).unwrap()
       .split(":")
       .nth(1).unwrap()
       .split(" ")
       .filter(|x| !x.is_empty())
       .map(|x| x.parse::<u64>().unwrap())
       .collect(); 

   println!("{:?}", &times);
   println!("{:?}", &records);

   // t -> (var) how much time to hold the button
   // T -> (const) time for each race
   // z -> (const) record for each race
   // d(t) -> distance
   // d(t) = t * (T - t) = t * T - t^2
   // Find t in 0..T such that
   // d(t) > z <=> -t^2 + T * t - z > 0
   let mut n: u64 = 1;
   for (&t_race, &z) in times.iter().zip(records.iter()) {
       let disc = (t_race * t_race) as i64 - 4 * z as i64;
       let mut t0 = 0f64;
       let mut t1 = 0f64;
       if disc < 0 {
           continue
       } else if disc == 0 {
           t0 = 0.5 * (t_race as f64 + (disc as f64).sqrt());
           t1 = t0;
       } else {
           t0 = 0.5 * (t_race as f64 - (disc as f64).sqrt());
           t1 = 0.5 * (t_race as f64 + (disc as f64).sqrt());
           let nn = (t1 - t0.floor()).ceil() as u64 - 1;
           n *= nn;
       }
   }
   println!("{}", n);
}

impl Solution for Day06 {
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

    fn part2(&self, input: &String) -> Result<i32, String> {
        Ok(5)
    }
}
