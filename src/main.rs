use aoc23::days::{*, common::Solution};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    day: i8,
    input_file: String,
}

fn get_day(day: i8) -> impl Solution {
    match day {
        1 => day01::Day01::new(),
        _ => unimplemented!()
    }
}


fn main() {

    let cli = Cli::parse();
    let d =get_day(cli.day);
    match d.run(cli.input_file) {
        Err(e) => println!("Error parsing file: {}", e),
        Ok((r1, r2)) => {
            if let Ok(r) = r1 {
                println!("Part 1 result: {}", r);
            }
            if let Ok(r) = r2 {
                println!("Part 2 result: {}", r);
            }
        }
    }
    
}
