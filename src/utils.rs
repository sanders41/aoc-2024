use std::{
    env::current_dir,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::Day;

pub fn build_twod_vec(lines: Lines<BufReader<File>>) -> Result<Vec<Vec<char>>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines.map_while(Result::ok) {
        let columns = line.chars().collect::<Vec<char>>();
        grid.push(columns);
    }

    Ok(grid)
}

pub fn build_twod_vec_usize(lines: Lines<BufReader<File>>) -> Result<Vec<Vec<usize>>> {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    for line in lines.map_while(Result::ok) {
        let columns = line
            .chars()
            .map(|s| s.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        grid.push(columns);
    }

    Ok(grid)
}

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn build_data_file_path(day: &Day, file_name: &str) -> Result<PathBuf> {
    let mut built_path = current_dir().unwrap();
    built_path.push(format!("src/{day}/{file_name}"));

    Ok(built_path)
}

pub fn split_whitespace_to_usize(value: &str) -> Vec<usize> {
    value
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

pub fn is_in_bounds<T>(grid: &[Vec<T>], x: isize, y: isize) -> bool {
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
    fn test_read_lines() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = "1\n2\n";
        file.write_all(data.as_bytes()).unwrap();
        let lines: Vec<String> = read_lines(&file_path)
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(lines, vec!["1", "2"]);
    }

    #[test]
    fn test_build_data_file_path() {
        let mut expected = current_dir().unwrap();
        expected.push("src/day1/data.txt");
        let result = build_data_file_path(&Day::Day1, "data.txt").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_bounds() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let result = is_in_bounds(&grid, 0, 1);

        assert!(result);
    }

    #[test]
    fn test_is_out_of_bounds() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let result = is_in_bounds(&grid, 0, 9);

        assert!(!result);
    }
}
