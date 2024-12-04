use std::{
    fs::File,
    io::{BufReader, Lines},
};

use anyhow::Result;

use crate::{
    utils::{build_data_file_path, read_lines},
    Day,
};

pub fn puzzle1() {
    let file_path = build_data_file_path(&Day::Day4, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_one(lines).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Day::Day4, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_two(lines).unwrap();

    println!("{result}");
}

fn calculate_part_one(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;
    let word_chars = ['X', 'M', 'A', 'S'];
    let grid = build_vec(lines).unwrap();
    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            if grid[row][column] != 'X' {
                continue;
            }

            for (x, y) in &directions {
                let mut found = true;
                for (i, _) in word_chars.iter().enumerate() {
                    let row_check = row as isize + i as isize * x;
                    let column_check = column as isize + i as isize * y;

                    if row_check < 0
                        || column_check < 0
                        || row_check >= grid.len() as isize
                        || column_check >= grid.len() as isize
                        || grid[row_check as usize][column_check as usize] != word_chars[i]
                    {
                        found = false;
                        break;
                    }
                }

                if found {
                    total += 1;
                }
            }
        }
    }

    Ok(total)
}

fn calculate_part_two(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;
    let grid = build_vec(lines).unwrap();

    for row in 1..grid.len() - 1 {
        for column in 1..grid[row].len() - 1 {
            if grid[row][column] == 'A' {
                let top_left_to_bottom_right =
                    grid[row - 1][column - 1] == 'M' && grid[row + 1][column + 1] == 'S';

                let top_right_to_bottom_left =
                    grid[row - 1][column + 1] == 'M' && grid[row + 1][column - 1] == 'S';

                let bottom_left_to_top_right =
                    grid[row + 1][column - 1] == 'M' && grid[row - 1][column + 1] == 'S';

                let bottom_right_to_top_left =
                    grid[row + 1][column + 1] == 'M' && grid[row - 1][column - 1] == 'S';

                if !(!top_right_to_bottom_left && !bottom_left_to_top_right
                    || !top_left_to_bottom_right && !bottom_right_to_top_left)
                {
                    total += 1;
                }
            }
        }
    }

    Ok(total)
}

fn build_vec(lines: Lines<BufReader<File>>) -> Result<Vec<Vec<char>>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines.map_while(Result::ok) {
        let columns = line.chars().collect::<Vec<char>>();
        grid.push(columns);
    }

    Ok(grid)
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
        let data = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_one(lines).unwrap();

        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_two() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_two(lines).unwrap();

        assert_eq!(result, 9);
    }
}
