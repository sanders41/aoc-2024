use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Lines},
};

use anyhow::Result;

use crate::{
    utils::{build_data_file_path, build_twod_vec_usize, read_lines},
    Day,
};

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn puzzle1() {
    let file_path = build_data_file_path(&Day::Day10, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_one(lines).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Day::Day10, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_two(lines).unwrap();

    println!("{result}");
}

fn calculate_part_one(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;
    let grid = build_twod_vec_usize(lines).unwrap();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 0 {
                total += count_paths(&grid, row, col, false);
            }
        }
    }

    Ok(total)
}

fn calculate_part_two(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;
    let grid = build_twod_vec_usize(lines).unwrap();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 0 {
                total += count_paths(&grid, row, col, true);
            }
        }
    }

    Ok(total)
}

fn count_paths(grid: &[Vec<usize>], row: usize, col: usize, ignore_visited: bool) -> usize {
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    let mut found = 0;

    stack.push((row, col, 0));
    while let Some((x, y, level)) = stack.pop() {
        if ignore_visited || visited.insert((x, y)) {
            if level == 9 {
                found += 1;
                continue;
            }

            for (dx, dy) in DIRECTIONS {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;

                if is_in_bounds(grid, new_x, new_y)
                    && grid[new_x as usize][new_y as usize] == level + 1
                {
                    stack.push((new_x as usize, new_y as usize, level + 1));
                }
            }
        }
    }

    found
}

fn is_in_bounds(grid: &[Vec<usize>], x: isize, y: isize) -> bool {
    x < grid.len() as isize && x >= 0 && y < grid[0].len() as isize && y >= 0
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
        let data = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_one(lines).unwrap();

        assert_eq!(result, 36);
    }

    #[test]
    fn test_part_two() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_two(lines).unwrap();

        assert_eq!(result, 81);
    }
}
