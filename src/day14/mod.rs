use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
};

use anyhow::Result;
use regex::Regex;

use crate::{
    utils::{build_data_file_path, read_lines},
    Day,
};

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    fn quadrent_one(&self) -> HashMap<&str, (usize, usize)> {
        let mut quad = HashMap::new();
        let width = (self.width as f64 / 2.0 - 2.0).ceil() as usize;
        quad.insert("width", (0, width));

        let height = (self.height as f64 / 2.0 - 2.0).ceil() as usize;
        quad.insert("height", (0, height));

        quad
    }

    fn quadrent_two(&self) -> HashMap<&str, (usize, usize)> {
        let mut quad = HashMap::new();

        let width_start = (self.width as f64 / 2.0).ceil() as usize;
        quad.insert("width", (width_start, self.width - 1));

        let height = (self.height as f64 / 2.0 - 2.0).ceil() as usize;
        quad.insert("height", (0, height));

        quad
    }

    fn quadrent_three(&self) -> HashMap<&str, (usize, usize)> {
        let mut quad = HashMap::new();

        let width = (self.width as f64 / 2.0 - 2.0).ceil() as usize;
        quad.insert("width", (0, width));

        let height_start = (self.height as f64 / 2.0).ceil() as usize;
        quad.insert("height", (height_start, self.height - 1));

        quad
    }

    fn quadrent_four(&self) -> HashMap<&str, (usize, usize)> {
        let mut quad = HashMap::new();

        let width_start = (self.width as f64 / 2.0).ceil() as usize;
        quad.insert("width", (width_start, self.width - 1));

        let height_start = (self.height as f64 / 2.0).ceil() as usize;
        quad.insert("height", (height_start, self.height - 1));

        quad
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn new(position: (isize, isize), velocity: (isize, isize)) -> Self {
        Self { position, velocity }
    }

    fn move_robot(&mut self, grid: &Grid, seconds: usize) {
        for _ in 0..seconds {
            let mut new_width = self.position.0 + self.velocity.0;
            let mut new_height = self.position.1 + self.velocity.1;

            if new_width < 0 {
                new_width += grid.width as isize;
            }

            if new_width >= grid.width as isize {
                new_width -= grid.width as isize;
            }

            if new_height < 0 {
                new_height += grid.height as isize;
            }

            if new_height >= grid.height as isize {
                new_height -= grid.height as isize;
            }

            self.position = (new_width, new_height);
        }
    }
}

pub fn puzzle1() {
    let file_path = build_data_file_path(&Day::Day14, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let grid = Grid::new(101, 103);
    let seconds = 100;
    let result = calculate_part_one(lines, &grid, seconds).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Day::Day14, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let grid = Grid::new(101, 103);
    let seconds = 1;
    let result = calculate_part_two(lines, &grid, seconds).unwrap();

    println!("{result}");
}

fn calculate_part_one(lines: Lines<BufReader<File>>, grid: &Grid, seconds: usize) -> Result<usize> {
    let mut quadrent_one = 0;
    let mut quadrent_two = 0;
    let mut quadrent_three = 0;
    let mut quadrent_four = 0;
    let grid_quadrent_one = grid.quadrent_one();
    let grid_quadrent_two = grid.quadrent_two();
    let grid_quadrent_three = grid.quadrent_three();
    let grid_quadrent_four = grid.quadrent_four();

    for line in lines.map_while(Result::ok) {
        let mut robot = get_robot(&line).unwrap();
        robot.move_robot(grid, seconds);
        if (grid_quadrent_one.get("width").unwrap().0 as isize
            ..=grid_quadrent_one.get("width").unwrap().1 as isize)
            .contains(&robot.position.0)
            && (grid_quadrent_one.get("height").unwrap().0 as isize
                ..=grid_quadrent_one.get("height").unwrap().1 as isize)
                .contains(&robot.position.1)
        {
            quadrent_one += 1;
        } else if (grid_quadrent_two.get("width").unwrap().0 as isize
            ..=grid_quadrent_two.get("width").unwrap().1 as isize)
            .contains(&robot.position.0)
            && (grid_quadrent_two.get("height").unwrap().0 as isize
                ..=grid_quadrent_two.get("height").unwrap().1 as isize)
                .contains(&robot.position.1)
        {
            quadrent_two += 1;
        } else if (grid_quadrent_three.get("width").unwrap().0 as isize
            ..=grid_quadrent_three.get("width").unwrap().1 as isize)
            .contains(&robot.position.0)
            && (grid_quadrent_three.get("height").unwrap().0 as isize
                ..=grid_quadrent_three.get("height").unwrap().1 as isize)
                .contains(&robot.position.1)
        {
            quadrent_three += 1;
        } else if (grid_quadrent_four.get("width").unwrap().0 as isize
            ..=grid_quadrent_four.get("width").unwrap().1 as isize)
            .contains(&robot.position.0)
            && (grid_quadrent_four.get("height").unwrap().0 as isize
                ..=grid_quadrent_four.get("height").unwrap().1 as isize)
                .contains(&robot.position.1)
        {
            quadrent_four += 1;
        }
    }

    let total = quadrent_one * quadrent_two * quadrent_three * quadrent_four;
    Ok(total)
}

/// This code is trash. It doesn't really answer the qustions. I just printed it and search for
/// something I thought might be the answer.
fn calculate_part_two(lines: Lines<BufReader<File>>, grid: &Grid, seconds: usize) -> Result<usize> {
    let mut robots = Vec::new();
    let mut total = 0;

    for line in lines.map_while(Result::ok) {
        let robot = get_robot(&line).unwrap();
        robots.push(robot);
    }

    let mut checking = true;

    while checking {
        total += 1;
        let mut tree = vec![vec!['.'; grid.height]; grid.width];

        for robot in &mut robots {
            robot.move_robot(grid, seconds);
            if (0..tree.len()).contains(&(robot.position.1 as usize))
                && (0..tree[0].len()).contains(&(robot.position.0 as usize))
            {
                tree[robot.position.1 as usize][robot.position.0 as usize] = 'x';
            }
        }

        println!("{:?}", tree);
        println!("total: {total}");

        if total == 10000 {
            checking = false;
        }
    }

    Ok(total)
}

fn get_robot(line: &str) -> Result<Robot> {
    let re = Regex::new(r"(-?\d+),(-?\d+)")?;

    let pairs = re
        .captures_iter(line)
        .map(|capture| {
            let x = capture[1].parse::<isize>().unwrap();
            let y = capture[2].parse::<isize>().unwrap();
            (x, y)
        })
        .collect::<Vec<(isize, isize)>>();

    Ok(Robot::new(pairs[0], pairs[1]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::{create_dir_all, File},
        io::prelude::*,
    };
    use tempfile::tempdir;

    #[test]
    fn test_part_one() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let grid = Grid::new(11, 7);
        let seconds = 100;
        let result = calculate_part_one(lines, &grid, seconds).unwrap();

        assert_eq!(result, 12);
    }

    /*#[test]
    fn test_part_two() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let grid = Grid::new(11, 7);
        let seconds = 100;
        let result = calculate_part_two(lines, &grid, 100).unwrap();

        assert_eq!(result, 0);
    }*/

    #[test]
    fn test_get_robot() {
        let result = get_robot("p=0,4 v=3,-3").unwrap();

        assert_eq!(result, Robot::new((0, 4), (3, -3)));
    }

    #[test]
    fn test_move_robot() {
        let grid = Grid::new(11, 7);
        let mut result = get_robot("p=2,4 v=2,-3").unwrap();
        result.move_robot(&grid, 5);
        let expected = Robot::new((1, 3), (2, -3));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_quadrant_one() {
        let grid = Grid::new(11, 7);
        let mut expected = HashMap::new();
        expected.insert("width", (0, 4));
        expected.insert("height", (0, 2));

        assert_eq!(grid.quadrent_one(), expected);
    }

    #[test]
    fn test_quadrant_two() {
        let grid = Grid::new(11, 7);
        let mut expected = HashMap::new();
        expected.insert("width", (6, 10));
        expected.insert("height", (0, 2));

        assert_eq!(grid.quadrent_two(), expected);
    }

    #[test]
    fn test_quadrant_three() {
        let grid = Grid::new(11, 7);
        let mut expected = HashMap::new();
        expected.insert("width", (0, 4));
        expected.insert("height", (4, 6));

        assert_eq!(grid.quadrent_three(), expected);
    }

    #[test]
    fn test_quadrant_four() {
        let grid = Grid::new(11, 7);
        let mut expected = HashMap::new();
        expected.insert("width", (6, 10));
        expected.insert("height", (4, 6));

        assert_eq!(grid.quadrent_four(), expected);
    }
}
