use std::fs::read_to_string;

use anyhow::Result;
use regex::Regex;

use crate::{utils::build_data_file_path, Day};

#[derive(Debug, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Hash)]
struct Game {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

impl Game {
    fn new(button_a: Position, button_b: Position, prize: Position) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    fn calculate_times_b(&self) -> f64 {
        (self.prize.y as f64 * self.button_a.x as f64
            - self.prize.x as f64 * self.button_a.y as f64)
            / (self.button_b.y as f64 * self.button_a.x as f64
                - self.button_b.x as f64 * self.button_a.y as f64)
    }

    fn calculate_times_a(&self) -> f64 {
        (self.prize.x as f64 - self.button_b.x as f64 * self.calculate_times_b())
            / self.button_a.x as f64
    }
}

pub fn puzzle1() {
    let file_path = build_data_file_path(&Day::Day13, "data.txt").unwrap();
    let data = read_to_string(file_path).unwrap();
    let result = calculate_part_one(&data).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Day::Day13, "data.txt").unwrap();
    let data = read_to_string(file_path).unwrap();
    let result = calculate_part_two(&data).unwrap();

    println!("{result}");
}

fn calculate_part_one(data: &str) -> Result<usize> {
    let mut total = 0;
    let games = data.split("\n\n").collect::<Vec<&str>>();

    for game in games {
        let game_parts = build_game(game).unwrap();
        let times_b = game_parts.calculate_times_b();
        let times_a = game_parts.calculate_times_a();

        if times_a == times_a.floor()
            && times_b == times_b.floor()
            && (0.0..=100.0).contains(&times_a)
            && (0.0..=100.0).contains(&times_b)
        {
            total += times_a as usize * 3 + times_b as usize;
        }
    }

    Ok(total)
}

fn calculate_part_two(data: &str) -> Result<usize> {
    let mut total = 0;
    let games = data.split("\n\n").collect::<Vec<&str>>();

    for game in games {
        let mut game_parts = build_game(game).unwrap();
        game_parts.prize.x += 10000000000000;
        game_parts.prize.y += 10000000000000;

        let times_b = game_parts.calculate_times_b();
        let times_a = game_parts.calculate_times_a();

        if times_a == times_a.floor() && times_b == times_b.floor() {
            total += times_a as usize * 3 + times_b as usize;
        }
    }

    Ok(total)
}

fn build_game(game_str: &str) -> Result<Game> {
    let parts = game_str.split("\n").collect::<Vec<&str>>();
    let rex = Regex::new(r"X\+\d+").unwrap();
    let rey = Regex::new(r"Y\+\d+").unwrap();
    let re_prizex = Regex::new(r"X=\d+").unwrap();
    let re_prizey = Regex::new(r"Y=\d+").unwrap();
    let button_a_x = rex
        .find_iter(parts[0])
        .next()
        .map(|x| x.as_str().split_once('+').unwrap())
        .and_then(|(_, num)| num.parse::<isize>().ok())
        .unwrap();
    let button_a_y = rey
        .find_iter(parts[0])
        .next()
        .map(|x| x.as_str().split_once('+').unwrap())
        .and_then(|(_, num)| num.parse::<isize>().ok())
        .unwrap();
    let button_a = Position::new(button_a_x, button_a_y);
    let button_b_x = rex
        .find_iter(parts[1])
        .next()
        .map(|x| x.as_str().split_once('+').unwrap())
        .and_then(|(_, num)| num.parse::<isize>().ok())
        .unwrap();
    let button_b_y = rey
        .find_iter(parts[1])
        .next()
        .map(|x| x.as_str().split_once('+').unwrap())
        .and_then(|(_, num)| num.parse::<isize>().ok())
        .unwrap();
    let button_b = Position::new(button_b_x, button_b_y);
    let prize_x = re_prizex
        .find_iter(parts[2])
        .next()
        .map(|x| x.as_str().split_once('=').unwrap())
        .and_then(|(_, num)| num.parse::<isize>().ok())
        .unwrap();
    let prize_y = re_prizey
        .find_iter(parts[2])
        .next()
        .map(|x| x.as_str().split_once('=').unwrap())
        .and_then(|(_, num)| num.parse::<isize>().ok())
        .unwrap();
    let prize = Position::new(prize_x, prize_y);
    let game = Game::new(button_a, button_b, prize);

    Ok(game)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        let result = calculate_part_one(&data).unwrap();

        assert_eq!(result, 480);
    }

    #[test]
    fn test_part_two() {
        let data = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        let result = calculate_part_two(&data).unwrap();

        assert_eq!(result, 875318608908);
    }
}
