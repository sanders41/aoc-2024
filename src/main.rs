mod utils;
mod year_2024;

use std::fmt::{Display, Formatter};

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

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Day::Day1 => Display::fmt("day1", f),
            Day::Day2 => Display::fmt("day2", f),
            Day::Day3 => Display::fmt("day3", f),
            Day::Day4 => Display::fmt("day4", f),
            Day::Day5 => Display::fmt("day5", f),
            Day::Day6 => Display::fmt("day6", f),
            Day::Day7 => Display::fmt("day7", f),
            Day::Day8 => Display::fmt("day8", f),
            Day::Day9 => Display::fmt("day9", f),
            Day::Day10 => Display::fmt("day10", f),
            Day::Day11 => Display::fmt("day11", f),
            Day::Day12 => Display::fmt("day12", f),
            Day::Day13 => Display::fmt("day13", f),
            Day::Day14 => Display::fmt("day14", f),
            Day::Day15 => Display::fmt("day15", f),
            Day::Day16 => Display::fmt("day16", f),
            Day::Day17 => Display::fmt("day17", f),
            Day::Day18 => Display::fmt("day18", f),
            Day::Day19 => Display::fmt("day19", f),
            Day::Day20 => Display::fmt("day20", f),
            Day::Day21 => Display::fmt("day21", f),
            Day::Day22 => Display::fmt("day22", f),
            Day::Day23 => Display::fmt("day23", f),
            Day::Day24 => Display::fmt("day24", f),
            Day::Day25 => Display::fmt("day25", f),
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
enum Year {
    Year2024,
}

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Year::Year2024 => Display::fmt("year_2024", f),
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
enum Puzzle {
    Puzzle1,
    Puzzle2,
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
