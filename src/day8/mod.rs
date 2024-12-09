use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Lines},
};

use anyhow::Result;

use crate::{
    utils::{build_data_file_path, build_twod_vec, read_lines},
    Day,
};

#[derive(Debug, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

pub fn puzzle1() {
    let file_path = build_data_file_path(&Day::Day8, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_one(lines).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Day::Day8, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_two(lines).unwrap();

    println!("{result}");
}

fn calculate_part_one(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;
    let grid = build_twod_vec(lines).unwrap();
    let grid_len = grid.len() as isize;
    let grid_width = grid[0].len() as isize;
    let mut positions: HashMap<char, Vec<Position>> = HashMap::new();
    let mut counted: HashSet<(isize, isize)> = HashSet::new();

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '.' {
                continue;
            }

            let current_position = Position { x, y };

            if let Some(positions) = positions.get(&grid[x][y]) {
                for position in positions {
                    let offset_one = (
                        2 * (position.x as isize - current_position.x as isize),
                        2 * (position.y as isize - current_position.y as isize),
                    );
                    let offset_two = (
                        2 * (current_position.x as isize - position.x as isize),
                        2 * (current_position.y as isize - position.y as isize),
                    );

                    if (position.x as isize - offset_one.0) >= 0
                        && (position.x as isize - offset_one.0) < grid_len
                        && (position.y as isize - offset_one.1) >= 0
                        && (position.y as isize - offset_one.1) < grid_width
                        && !counted.contains(&(
                            position.x as isize - offset_one.0,
                            position.y as isize - offset_one.1,
                        ))
                    {
                        total += 1;
                        counted.insert((
                            position.x as isize - offset_one.0,
                            position.y as isize - offset_one.1,
                        ));
                    }

                    if (current_position.x as isize - offset_two.0) >= 0
                        && (current_position.x as isize - offset_two.0) < grid_len
                        && (current_position.y as isize - offset_two.1) >= 0
                        && (current_position.y as isize - offset_two.1) < grid_width
                        && !counted.contains(&(
                            current_position.x as isize - offset_two.0,
                            current_position.y as isize - offset_two.1,
                        ))
                    {
                        total += 1;
                        counted.insert((
                            current_position.x as isize - offset_two.0,
                            current_position.y as isize - offset_two.1,
                        ));
                    }

                    if (current_position.x as isize - offset_two.0) >= 0
                        && (current_position.x as isize - offset_two.0) < grid_len
                        && (current_position.y as isize - offset_two.1) >= 0
                        && (current_position.y as isize - offset_two.1) < grid_width
                        && !counted.contains(&(
                            current_position.x as isize - offset_two.0,
                            current_position.y as isize - offset_two.1,
                        ))
                    {
                        total += 1;
                        counted.insert((
                            current_position.x as isize - offset_two.0,
                            current_position.y as isize - offset_two.1,
                        ));
                    }
                }
            }

            if let Some(positions) = positions.get_mut(&grid[x][y]) {
                positions.push(Position { x, y });
            } else {
                positions.insert(grid[x][y], vec![Position { x, y }]);
            }
        }
    }

    Ok(total)
}

fn calculate_part_two(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;
    let grid = build_twod_vec(lines).unwrap();
    let grid_len = grid.len() as isize;
    let grid_width = grid[0].len() as isize;
    let mut positions: HashMap<char, Vec<Position>> = HashMap::new();
    let mut counted: HashSet<(isize, isize)> = HashSet::new();

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '.' {
                continue;
            }

            let current_position = Position { x, y };

            if let Some(positions) = positions.get(&grid[x][y]) {
                for position in positions {
                    let offset_one = (
                        position.x as isize - current_position.x as isize,
                        position.y as isize - current_position.y as isize,
                    );
                    let offset_two = (
                        current_position.x as isize - position.x as isize,
                        current_position.y as isize - position.y as isize,
                    );

                    let mut check_x_position = position.x as isize - offset_one.0;
                    let mut check_y_position = position.y as isize - offset_one.1;

                    while check_x_position >= 0
                        && check_x_position < grid_len
                        && check_y_position >= 0
                        && check_y_position < grid_width
                    {
                        if !counted.contains(&(check_x_position, check_y_position)) {
                            total += 1;

                            counted.insert((check_x_position, check_y_position));
                        }

                        check_x_position -= offset_one.0;
                        check_y_position -= offset_one.1;
                    }

                    let mut check_x_current_position = current_position.x as isize - offset_two.0;
                    let mut check_y_current_position = current_position.y as isize - offset_two.1;

                    while check_x_current_position >= 0
                        && check_x_current_position < grid_len
                        && check_y_current_position >= 0
                        && check_y_current_position < grid_width
                    {
                        if !counted.contains(&(check_x_current_position, check_y_current_position))
                        {
                            total += 1;

                            counted.insert((check_x_current_position, check_y_current_position));
                        }

                        check_x_current_position -= offset_two.0;
                        check_y_current_position -= offset_two.1;
                    }
                }
            }

            if let Some(positions) = positions.get_mut(&grid[x][y]) {
                positions.push(Position { x, y });
            } else {
                positions.insert(grid[x][y], vec![Position { x, y }]);
            }
        }
    }

    Ok(total)
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
        let data = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_one(lines).unwrap();

        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_two() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_two(lines).unwrap();

        assert_eq!(result, 34);
    }
}
