use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufReader, Lines},
};

use anyhow::Result;

use crate::{
    utils::{build_data_file_path, read_lines, split_whitespace_to_usize},
    Day,
};

pub fn puzzle1() {
    let file_path = build_data_file_path(&Day::Day11, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_one(lines).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Day::Day11, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_two(lines).unwrap();

    println!("{result}");
}

fn calculate_part_one(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut numbers = VecDeque::new();

    for line in lines.map_while(Result::ok) {
        let nums = split_whitespace_to_usize(&line);
        for num in nums {
            numbers.push_back(num);
        }
    }

    let numbers = do_calculations_part_one(&mut numbers, 25);

    Ok(numbers.len())
}

fn calculate_part_two(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;
    let mut tracker = HashMap::new();
    let rounds = 75;

    for line in lines.map_while(Result::ok) {
        let nums = split_whitespace_to_usize(&line);
        for num in nums {
            total += do_calculations_part_two(num, rounds, &mut tracker);
        }
    }

    Ok(total)
}

fn do_calculations_part_two(
    value: usize,
    remaining: usize,
    tracker: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if tracker.contains_key(&(value, remaining)) {
        tracker[&(value, remaining)]
    } else if remaining == 0 {
        1
    } else {
        let count = if value == 0 {
            do_calculations_part_two(1, remaining - 1, tracker)
        } else if value.to_string().len() % 2 == 0 {
            let values = split_num(value);
            [values.0, values.1]
                .iter()
                .map(|v| do_calculations_part_two(*v, remaining - 1, tracker))
                .sum()
        } else {
            do_calculations_part_two(value * 2024, remaining - 1, tracker)
        };

        tracker.insert((value, remaining), count);
        count
    }
}

fn do_calculations_part_one(numbers: &mut VecDeque<usize>, rounds: usize) -> &mut VecDeque<usize> {
    for _ in 0..rounds {
        let current_len = numbers.len();

        for _ in 0..current_len {
            let num = numbers.pop_back().unwrap();
            if num == 0 {
                numbers.push_front(num + 1);
            } else if num.to_string().len() % 2 == 0 {
                let (v1, v2) = split_num(num);
                numbers.push_front(v2);
                numbers.push_front(v1);
            } else {
                numbers.push_front(num * 2024);
            }
        }
    }

    numbers
}

fn split_num(num: usize) -> (usize, usize) {
    let mut temp = num.to_string();
    let split = temp.len() / 2;
    let (v1, v2) = temp.split_at_mut(split);
    (v1.parse::<usize>().unwrap(), v2.parse::<usize>().unwrap())
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
        let data = r#"125 17"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_one(lines).unwrap();

        assert_eq!(result, 55312);
    }

    #[test]
    fn test_part_two() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"125 17"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_two(lines).unwrap();

        assert_eq!(result, 65601038650482);
    }

    #[test]
    fn test_split_num() {
        let result = split_num(1000);

        assert_eq!(result, (10, 0));
    }
}
