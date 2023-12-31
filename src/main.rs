use aoc23::days::{common::Solution, *};
use clap::Parser;
use std::time::Instant;

#[derive(Parser)]
struct Cli {
    day: i8,
    input_file: String,
    part: Option<u8>,
}

fn get_day(
    day: i8,
) -> Box<dyn Solution<ParsedInput = String, Part1Output = i32, Part2Output = i32>> {
    match day {
        1 => Box::new(day01::Day01::new()),
        2 => Box::new(day02::Day02::new()),
        3 => Box::new(day03::Day03::new()),
        4 => Box::new(day04::Day04::new()),
        5 => Box::new(day05::Day05::new()),
        6 => Box::new(day06::Day06::new()),
        7 => Box::new(day07::Day07::new()),
        8 => Box::new(day08::Day08::new()),
        9 => Box::new(day09::Day09::new()),
        10 => Box::new(day10::Day10::new()),
        11 => Box::new(day11::Day11::new()),
        12 => Box::new(day12::Day12::new()),
        13 => Box::new(day13::Day13::new()),
        14 => Box::new(day14::Day14::new()),
        15 => Box::new(day15::Day15::new()),
        16 => Box::new(day16::Day16::new()),
        17 => Box::new(day17::Day17::new()),
        18 => Box::new(day18::Day18::new()),
        19 => Box::new(day19::Day19::new()),
        20 => Box::new(day20::Day20::new()),
        21 => Box::new(day21::Day21::new()),
        22 => Box::new(day22::Day22::new()),
        23 => Box::new(day23::Day23::new()),
        24 => Box::new(day24::Day24::new()),
        25 => Box::new(day25::Day25::new()),
        _ => unimplemented!(),
    }
}

fn main() {
    let cli = Cli::parse();
    let d = get_day(cli.day);
    let start_time = Instant::now();
    let res = d.run(cli.input_file, cli.part);
    let elapsed = start_time.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    match res {
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
