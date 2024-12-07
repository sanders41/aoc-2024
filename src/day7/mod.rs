use std::{
    collections::VecDeque,
    fs::File,
    io::{BufReader, Lines},
};

use anyhow::Result;

use crate::{
    utils::{build_data_file_path, read_lines, split_whitespace_to_usize},
    Day,
};

pub fn puzzle1() {
    let file_path = build_data_file_path(&Day::Day7, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_one(lines).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Day::Day7, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_two(lines).unwrap();

    println!("{result}");
}

fn calculate_part_one(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;

    for line in lines.map_while(Result::ok) {
        let (answer_str, values_str) = line.split_once(": ").unwrap();
        let answer = answer_str.parse::<usize>().unwrap();
        let values = split_whitespace_to_usize(values_str);

        if let Some(valid) = get_valid(&values, &answer) {
            total += valid;
        }
    }

    Ok(total)
}

fn calculate_part_two(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;

    for line in lines.map_while(Result::ok) {
        let (answer_str, values_str) = line.split_once(": ").unwrap();
        let answer = answer_str.parse::<usize>().unwrap();
        let values = split_whitespace_to_usize(values_str);

        if let Some(valid) = get_valid_with_concat(&values, &answer) {
            total += valid;
        }
    }

    Ok(total)
}

fn get_valid(values: &[usize], answer: &usize) -> Option<usize> {
    if &values.iter().copied().sum::<usize>() == answer {
        return Some(*answer);
    }

    let mut stack = VecDeque::new();
    stack.push_back((values[0], 1));

    while let Some((current_value, index)) = stack.pop_front() {
        if index == values.len() {
            if &current_value == answer {
                return Some(*answer);
            }
            continue;
        }

        let next_num = values[index];

        stack.push_back((current_value + next_num, index + 1));
        stack.push_back((current_value * next_num, index + 1));
    }

    None
}

fn get_valid_with_concat(values: &[usize], answer: &usize) -> Option<usize> {
    let mut stack = VecDeque::new();
    stack.push_back((values[0], 1));

    while let Some((current_value, index)) = stack.pop_front() {
        if index == values.len() {
            if &current_value == answer {
                return Some(*answer);
            }
            continue;
        }

        let next_num = values[index];

        stack.push_back((current_value + next_num, index + 1));
        stack.push_back((current_value * next_num, index + 1));

        let concatenated = format!("{}{}", current_value, next_num)
            .parse::<usize>()
            .unwrap();
        stack.push_back((concatenated, index + 1));
    }

    None
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
        let data = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_one(lines).unwrap();

        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_two() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_two(lines).unwrap();

        assert_eq!(result, 11387);
    }
}
