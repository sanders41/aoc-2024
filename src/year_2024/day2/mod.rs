use std::{
    fs::File,
    io::{BufReader, Lines},
};

use anyhow::Result;

use crate::{
    utils::{build_data_file_path, read_lines},
    Day, Year,
};

pub fn puzzle1() {
    let file_path = build_data_file_path(&Year::Year2024, &Day::Day2, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_one(lines).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Year::Year2024, &Day::Day2, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_two(lines).unwrap();

    println!("{result}");
}

fn calculate_part_one(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;

    for line in lines.map_while(Result::ok) {
        let levels = line
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if is_valid(&levels) {
            total += 1;
        }
    }

    Ok(total)
}

fn calculate_part_two(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;

    for line in lines.map_while(Result::ok) {
        let levels = line
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if is_valid(&levels) {
            total += 1;
        } else {
            for i in 0..levels.len() {
                let reduced = levels
                    .iter()
                    .enumerate()
                    .filter(|&(index, _)| index != i)
                    .map(|(_, &value)| value)
                    .collect::<Vec<_>>();

                if is_valid(&reduced) {
                    total += 1;
                    break;
                }
            }
        }
    }

    Ok(total)
}

fn is_valid(values: &[usize]) -> bool {
    if values.len() < 2 {
        return true;
    }
    let is_valid_increasing = values
        .windows(2)
        .all(|w| w[1] > w[0] && w[1].abs_diff(w[0]) <= 3);
    let is_valid_decreasing = values
        .windows(2)
        .all(|w| w[1] < w[0] && w[1].abs_diff(w[0]) <= 3);

    is_valid_increasing || is_valid_decreasing
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
        let data = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_one(lines).unwrap();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_two(lines).unwrap();

        assert_eq!(result, 4);
    }
}
