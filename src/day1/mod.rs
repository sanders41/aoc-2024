use std::{
    fs::File,
    io::{BufReader, Lines},
};

use anyhow::{bail, Result};

use crate::{
    utils::{build_data_file_path, read_lines},
    Day,
};

struct Values {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl Values {
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
        }
    }
}

pub fn puzzle1() {
    let file_path = build_data_file_path(&Day::Day1, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_one(lines).unwrap();
    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Day::Day1, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_two(lines).unwrap();
    println!("{result}");
}

fn calculate_part_one(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut values = gather_values(lines).unwrap();
    values.left.sort();
    values.right.sort();

    let total = values
        .left
        .iter()
        .zip(&values.right)
        .map(|(left, right)| left.abs_diff(*right))
        .sum();

    Ok(total)
}

fn calculate_part_two(lines: Lines<BufReader<File>>) -> Result<usize> {
    let values = gather_values(lines).unwrap();

    let total = values
        .left
        .iter()
        .map(|value| {
            let count = values.right.iter().filter(|&x| x == value).count();
            value * count
        })
        .sum();

    Ok(total)
}

fn gather_values(lines: Lines<BufReader<File>>) -> Result<Values> {
    let mut values = Values::new();

    for line in lines.map_while(Result::ok) {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 2 {
            bail!("Incorrect number of values in line: {line}");
        }
        let left_val = parts[0].parse::<usize>().unwrap();
        values.left.push(left_val);
        let right_val = parts[1].parse::<usize>().unwrap();
        values.right.push(right_val);
    }

    Ok(values)
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
        let data = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_one(lines).unwrap();

        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_two() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_two(lines).unwrap();

        assert_eq!(result, 31);
    }
}
