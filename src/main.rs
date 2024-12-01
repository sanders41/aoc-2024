mod utils;
mod year_2024;

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

#[derive(Clone, Debug, ValueEnum)]
enum Puzzle {
    Puzzle1,
    Puzzle2,
}

#[derive(Clone, Debug, ValueEnum)]
enum Year {
    Year2024,
}

#[derive(Debug, Parser)]
#[clap(author, version, about = "Run the puzzle file")]
struct Args {
    /// The aoc year to run
    year: Year,

    /// The day to run
    day: Day,

    /// The puzzle to run
    puzzle: Puzzle,
}

fn main() {
    let args = Args::parse();

    match args.year {
        Year::Year2024 => match args.day {
            Day::Day1 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day1::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day1::puzzle2(),
            },
            Day::Day2 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day2::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day2::puzzle2(),
            },
            Day::Day3 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day3::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day3::puzzle2(),
            },
            Day::Day4 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day4::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day4::puzzle2(),
            },
            Day::Day5 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day5::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day5::puzzle2(),
            },
            Day::Day6 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day6::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day6::puzzle2(),
            },
            Day::Day7 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day7::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day7::puzzle2(),
            },
            Day::Day8 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day8::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day8::puzzle2(),
            },
            Day::Day9 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day9::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day9::puzzle2(),
            },
            Day::Day10 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day10::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day10::puzzle2(),
            },
            Day::Day11 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day11::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day11::puzzle2(),
            },
            Day::Day12 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day12::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day12::puzzle2(),
            },
            Day::Day13 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day13::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day13::puzzle2(),
            },
            Day::Day14 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day14::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day14::puzzle2(),
            },
            Day::Day15 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day15::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day15::puzzle2(),
            },
            Day::Day16 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day16::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day16::puzzle2(),
            },
            Day::Day17 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day17::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day17::puzzle2(),
            },
            Day::Day18 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day18::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day18::puzzle2(),
            },
            Day::Day19 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day19::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day19::puzzle2(),
            },
            Day::Day20 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day20::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day20::puzzle2(),
            },
            Day::Day21 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day21::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day21::puzzle2(),
            },
            Day::Day22 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day22::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day22::puzzle2(),
            },
            Day::Day23 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day23::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day23::puzzle2(),
            },
            Day::Day24 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day24::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day24::puzzle2(),
            },
            Day::Day25 => match args.puzzle {
                Puzzle::Puzzle1 => year_2024::day25::puzzle1(),
                Puzzle::Puzzle2 => year_2024::day25::puzzle2(),
            },
        },
    }
}
