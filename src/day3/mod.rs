use std::fs::read_to_string;

use anyhow::Result;
use regex::Regex;

use crate::{utils::build_data_file_path, Day};

pub fn puzzle1() {
    let file_path = build_data_file_path(&Day::Day3, "data.txt").unwrap();
    let data = read_to_string(file_path).unwrap();
    let result = calculate_part_one(&data).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Day::Day3, "data.txt").unwrap();
    let data = read_to_string(file_path).unwrap();
    let result = calculate_part_two(&data).unwrap();

    println!("{result}");
}

fn calculate_part_one(data: &str) -> Result<usize> {
    let mut total = 0;
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    for values in re.find_iter(data) {
        let values_str = values.as_str();
        let stripped = values_str.replace("mul(", "").replace(')', "");
        let nums = stripped
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        total += nums[0] * nums[1];
    }

    Ok(total)
}

fn calculate_part_two(data: &str) -> Result<usize> {
    let mut total = 0;
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let parts = data.split("don't");
    for (index, part) in parts.enumerate() {
        if index == 0 {
            for values in re.find_iter(part) {
                let values_str = values.as_str();
                let stripped = values_str.replace("mul(", "").replace(')', "");
                let nums = stripped
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                total += nums[0] * nums[1];
            }
        } else if let Some(start) = part.find("do") {
            for values in re.find_iter(&part[start..]) {
                let values_str = values.as_str();
                let stripped = values_str.replace("mul(", "").replace(')', "");
                let nums = stripped
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                println!("{:?}", nums);
                total += nums[0] * nums[1];
            }
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = calculate_part_one(data).unwrap();

        assert_eq!(result, 161);
    }

    #[test]
    fn test_part_two() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = calculate_part_two(data).unwrap();

        assert_eq!(result, 48);
    }
}
