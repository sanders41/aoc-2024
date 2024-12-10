use std::fs::read_to_string;

use anyhow::Result;

use crate::{utils::build_data_file_path, Day};

pub fn puzzle1() {
    let file_path = build_data_file_path(&Day::Day9, "data.txt").unwrap();
    let data = read_to_string(file_path).unwrap().trim().to_owned();
    let result = calculate_part_one(&data).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Day::Day9, "data.txt").unwrap();
    let data = read_to_string(file_path).unwrap().trim().to_owned();
    let result = calculate_part_two(&data).unwrap();

    println!("{result}");
}

fn calculate_part_one(data: &str) -> Result<usize> {
    let mut disk = build_disk(data).unwrap();

    loop {
        let last_block = disk.iter().rposition(|c| c != ".");
        let first_empty = disk.iter().position(|c| c == ".");

        if last_block.is_none()
            || first_empty.is_none()
            || last_block.unwrap() < first_empty.unwrap()
        {
            break;
        }

        disk.swap(last_block.unwrap(), first_empty.unwrap());
    }

    let checksum = calculate_checksum(&disk).unwrap();
    Ok(checksum)
}

fn calculate_part_two(data: &str) -> Result<usize> {
    let mut drive: Vec<Option<usize>> = Vec::new();
    let mut curr_index: usize = 0;
    let mut size: Vec<usize> = vec![0; data.chars().count()];
    let mut loc: Vec<usize> = vec![0; data.chars().count()];

    for (index, block) in data.chars().enumerate() {
        if index % 2 == 0 {
            loc[curr_index] = drive.len();
            size[curr_index] = block.to_digit(10).unwrap() as usize;
            for _ in 0..block.to_digit(10).unwrap() {
                drive.push(Some(curr_index));
            }
            curr_index += 1;
        } else {
            for _ in 0..block.to_digit(10).unwrap() {
                drive.push(None);
            }
        }
    }

    let mut to_move = size.len() - 1;
    while to_move > 0 {
        let mut free_space: usize = 0;
        let mut first_free: usize = 0;

        while first_free < loc[to_move] && free_space < size[to_move] {
            first_free += free_space;
            free_space = 0;
            while drive[first_free].is_some() {
                first_free += 1
            }
            while first_free + free_space < drive.len() && drive[first_free + free_space].is_none()
            {
                free_space += 1
            }
        }

        if first_free >= loc[to_move] {
            to_move -= 1;
            continue;
        }

        for block in drive.iter_mut().skip(first_free).take(size[to_move]) {
            *block = Some(to_move as usize);
        }
        for block in drive.iter_mut().skip(loc[to_move]).take(size[to_move]) {
            *block = None;
        }
        to_move -= 1;
    }

    let mut result: usize = 0;
    for (index, block) in drive.iter().enumerate() {
        if block.is_some() {
            result += index * block.unwrap();
        } else {
            continue;
        }
    }

    Ok(result)
}

fn build_disk(disk_map: &str) -> Result<Vec<String>> {
    let mut disk = Vec::new();
    let mut current = 0;

    for (index, digit_char) in disk_map.chars().enumerate() {
        let digit = digit_char.to_digit(10).unwrap();
        let free_space = index % 2 != 0;

        for _ in 0..digit {
            if free_space {
                disk.push(".".to_string());
            } else {
                disk.push(current.to_string());
            }
        }

        if free_space {
            current += 1;
        }
    }

    Ok(disk)
}

fn calculate_checksum(disk: &[String]) -> Result<usize> {
    let mut checksum = 0;
    for (index, block) in disk
        .iter()
        .filter(|&c| c.clone() != ".")
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .iter()
        .enumerate()
    {
        let multiple = block * index;
        checksum += multiple;
    }

    Ok(checksum)
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
        let data = "2333133121414131402";
        file.write_all(data.as_bytes()).unwrap();
        let data = read_to_string(file_path).unwrap();
        let result = calculate_part_one(&data).unwrap();

        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_two() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = "2333133121414131402";
        file.write_all(data.as_bytes()).unwrap();
        let data = read_to_string(file_path).unwrap();
        let result = calculate_part_two(&data).unwrap();

        assert_eq!(result, 2858);
    }

    #[test]
    fn test_build_disk() {
        let map = "2333133121414131402";
        let result = build_disk(map).unwrap();
        let expected = vec![
            "0".to_string(),
            "0".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            "2".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            "3".to_string(),
            "3".to_string(),
            "3".to_string(),
            ".".to_string(),
            "4".to_string(),
            "4".to_string(),
            ".".to_string(),
            "5".to_string(),
            "5".to_string(),
            "5".to_string(),
            "5".to_string(),
            ".".to_string(),
            "6".to_string(),
            "6".to_string(),
            "6".to_string(),
            "6".to_string(),
            ".".to_string(),
            "7".to_string(),
            "7".to_string(),
            "7".to_string(),
            ".".to_string(),
            "8".to_string(),
            "8".to_string(),
            "8".to_string(),
            "8".to_string(),
            "9".to_string(),
            "9".to_string(),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_checksum() {
        let data = vec![
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "2".to_string(),
        ];
        let result = calculate_checksum(&data).unwrap();

        assert_eq!(result, 19);
    }
}
