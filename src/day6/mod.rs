use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Lines},
    sync::atomic::{AtomicUsize, Ordering},
};

use anyhow::Result;
use rayon::prelude::*;

use crate::{
    utils::{build_data_file_path, build_twod_vec, read_lines},
    Day,
};

pub fn puzzle1() {
    let file_path = build_data_file_path(&Day::Day6, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_one(lines).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Day::Day6, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_two(lines).unwrap();

    println!("{result}");
}

fn calculate_part_one(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;
    let grid = build_twod_vec(lines).unwrap();
    let turning_points = find_turning_points(&grid);

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '^' {
                let visits = traverse_grid(&grid, (x, y), 3, &turning_points);
                if visits > total {
                    total = visits
                }
            }
        }
    }

    Ok(total)
}

fn calculate_part_two(lines: Lines<BufReader<File>>) -> Result<usize> {
    let total = AtomicUsize::new(0);
    let grid = build_twod_vec(lines).unwrap();
    let turning_points = find_turning_points(&grid);
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(x, row)| {
            row.iter()
                .enumerate()
                .find(|&(_, &cell)| cell == '^')
                .map(|(y, _)| (x, y))
        })
        .unwrap();
    (0..grid.len()).into_par_iter().for_each(|x| {
        (0..grid[0].len()).into_par_iter().for_each(|y| {
            if grid[x][y] == '.' {
                let mut points = turning_points.clone();
                points.insert((x, y));
                if traverse_grid_cycle(&grid, start, 3, &points) {
                    total.fetch_add(1, Ordering::SeqCst);
                }
            }
        })
    });

    Ok(total.load(Ordering::SeqCst))
}

fn traverse_grid(
    grid: &[Vec<char>],
    start: (usize, usize),
    start_direction: usize,
    turning_points: &HashSet<(usize, usize)>,
) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut direction_index = start_direction;
    let mut position = start;
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut visit_count = 0;

    loop {
        let (x, y) = position;

        if !visited[x][y] {
            visited[x][y] = true;
            visit_count += 1;
        }

        let (dx, dy) = directions[direction_index];
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x < 0
            || new_y < 0
            || new_x >= grid.len() as isize
            || new_y >= grid[0].len() as isize
            || new_x == grid.len() as isize
        {
            break;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if turning_points.contains(&(new_x, new_y)) {
            if direction_index == directions.len() - 1 {
                position = (x, y);
                direction_index = 0;
                continue;
            } else {
                position = (x, y);
                direction_index += 1;
                continue;
            }
        }

        position = (new_x, new_y);
    }

    visit_count
}

fn traverse_grid_cycle(
    grid: &[Vec<char>],
    start: (usize, usize),
    start_direction: usize,
    turning_points: &HashSet<(usize, usize)>,
) -> bool {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut direction_index = start_direction;
    let mut position = start;
    let mut cycle_check = HashSet::new();

    loop {
        let (x, y) = position;

        if cycle_check.contains(&(position, direction_index)) {
            return true;
        }

        cycle_check.insert((position, direction_index));

        let (dx, dy) = directions[direction_index];
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x < 0
            || new_y < 0
            || new_x >= grid.len() as isize
            || new_y >= grid[0].len() as isize
            || new_x == grid.len() as isize
        {
            break;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if turning_points.contains(&(new_x, new_y)) {
            if direction_index == directions.len() - 1 {
                position = (x, y);
                direction_index = 0;
                continue;
            } else {
                position = (x, y);
                direction_index += 1;
                continue;
            }
        }

        position = (new_x, new_y);
    }

    false
}

fn find_turning_points(grid: &[Vec<char>]) -> HashSet<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &cell)| cell == '#')
                .map(move |(y, _)| (x, y))
        })
        .collect()
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
        let data = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_one(lines).unwrap();

        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_two() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_two(lines).unwrap();

        assert_eq!(result, 6);
    }
}
