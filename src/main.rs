mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

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
enum Puzzle {
    Puzzle1,
    Puzzle2,
}

#[derive(Debug, Parser)]
#[clap(author, version, about = "Run the puzzle file")]
struct Args {
    /// The day to run
    day: Day,

    /// The puzzle to run
    puzzle: Puzzle,

    /// Run the parallel version
    #[clap(short, long)]
    parallel: bool,
}

fn main() {
    let args = Args::parse();

    match args.day {
        Day::Day1 => match args.puzzle {
            Puzzle::Puzzle1 => {
                if !args.parallel {
                    day1::puzzle1();
                } else {
                    day1::puzzle1_parallel();
                }
            }
            Puzzle::Puzzle2 => {
                if !args.parallel {
                    day1::puzzle2();
                } else {
                    day1::puzzle2_parallel();
                }
            }
        },
        Day::Day2 => match args.puzzle {
            Puzzle::Puzzle1 => day2::puzzle1(),
            Puzzle::Puzzle2 => day2::puzzle2(),
        },
        Day::Day3 => match args.puzzle {
            Puzzle::Puzzle1 => day3::puzzle1(),
            Puzzle::Puzzle2 => day3::puzzle2(),
        },
        Day::Day4 => match args.puzzle {
            Puzzle::Puzzle1 => day4::puzzle1(),
            Puzzle::Puzzle2 => day4::puzzle2(),
        },
        Day::Day5 => match args.puzzle {
            Puzzle::Puzzle1 => day5::puzzle1(),
            Puzzle::Puzzle2 => day5::puzzle2(),
        },
        Day::Day6 => match args.puzzle {
            Puzzle::Puzzle1 => day6::puzzle1(),
            Puzzle::Puzzle2 => day6::puzzle2(),
        },
        Day::Day7 => match args.puzzle {
            Puzzle::Puzzle1 => day7::puzzle1(),
            Puzzle::Puzzle2 => day7::puzzle2(),
        },
        Day::Day8 => match args.puzzle {
            Puzzle::Puzzle1 => day8::puzzle1(),
            Puzzle::Puzzle2 => day8::puzzle2(),
        },
        Day::Day9 => match args.puzzle {
            Puzzle::Puzzle1 => day9::puzzle1(),
            Puzzle::Puzzle2 => day9::puzzle2(),
        },
        Day::Day10 => match args.puzzle {
            Puzzle::Puzzle1 => day10::puzzle1(),
            Puzzle::Puzzle2 => day10::puzzle2(),
        },
        Day::Day11 => match args.puzzle {
            Puzzle::Puzzle1 => day11::puzzle1(),
            Puzzle::Puzzle2 => day11::puzzle2(),
        },
        Day::Day12 => match args.puzzle {
            Puzzle::Puzzle1 => day12::puzzle1(),
            Puzzle::Puzzle2 => day12::puzzle2(),
        },
        Day::Day13 => match args.puzzle {
            Puzzle::Puzzle1 => day13::puzzle1(),
            Puzzle::Puzzle2 => day13::puzzle2(),
        },
        Day::Day14 => match args.puzzle {
            Puzzle::Puzzle1 => day14::puzzle1(),
            Puzzle::Puzzle2 => day14::puzzle2(),
        },
        Day::Day15 => match args.puzzle {
            Puzzle::Puzzle1 => day15::puzzle1(),
            Puzzle::Puzzle2 => day15::puzzle2(),
        },
        Day::Day16 => match args.puzzle {
            Puzzle::Puzzle1 => day16::puzzle1(),
            Puzzle::Puzzle2 => day16::puzzle2(),
        },
        Day::Day17 => match args.puzzle {
            Puzzle::Puzzle1 => day17::puzzle1(),
            Puzzle::Puzzle2 => day17::puzzle2(),
        },
        Day::Day18 => match args.puzzle {
            Puzzle::Puzzle1 => day18::puzzle1(),
            Puzzle::Puzzle2 => day18::puzzle2(),
        },
        Day::Day19 => match args.puzzle {
            Puzzle::Puzzle1 => day19::puzzle1(),
            Puzzle::Puzzle2 => day19::puzzle2(),
        },
        Day::Day20 => match args.puzzle {
            Puzzle::Puzzle1 => day20::puzzle1(),
            Puzzle::Puzzle2 => day20::puzzle2(),
        },
        Day::Day21 => match args.puzzle {
            Puzzle::Puzzle1 => day21::puzzle1(),
            Puzzle::Puzzle2 => day21::puzzle2(),
        },
        Day::Day22 => match args.puzzle {
            Puzzle::Puzzle1 => day22::puzzle1(),
            Puzzle::Puzzle2 => day22::puzzle2(),
        },
        Day::Day23 => match args.puzzle {
            Puzzle::Puzzle1 => day23::puzzle1(),
            Puzzle::Puzzle2 => day23::puzzle2(),
        },
        Day::Day24 => match args.puzzle {
            Puzzle::Puzzle1 => day24::puzzle1(),
            Puzzle::Puzzle2 => day24::puzzle2(),
        },
        Day::Day25 => match args.puzzle {
            Puzzle::Puzzle1 => day25::puzzle1(),
            Puzzle::Puzzle2 => day25::puzzle2(),
        },
    }
}
