use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
};

use anyhow::Result;

use crate::{
    utils::{build_data_file_path, read_lines},
    Day,
};

pub fn puzzle1() {
    let pairs_path = build_data_file_path(&Day::Day5, "pairs.txt").unwrap();
    let file_path = build_data_file_path(&Day::Day5, "data.txt").unwrap();
    let pairs = read_lines(pairs_path).unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_one(pairs, lines).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let pairs_path = build_data_file_path(&Day::Day5, "pairs.txt").unwrap();
    let pairs = read_lines(pairs_path).unwrap();
    let file_path = build_data_file_path(&Day::Day5, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_two(pairs, lines).unwrap();

    println!("{result}");
}

fn calculate_part_one(
    pairs: Lines<BufReader<File>>,
    lines: Lines<BufReader<File>>,
) -> Result<usize> {
    let mut total = 0;
    let mut pairs_vec = Vec::new();

    for pair in pairs.map_while(Result::ok) {
        let values = pair.split_once('|').unwrap();
        let x = values.0.parse::<usize>().unwrap();
        let y = values.1.parse::<usize>().unwrap();
        pairs_vec.push((x, y));
    }

    for line in lines.map_while(Result::ok) {
        let split = line
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if is_correct_order(&pairs_vec, split.clone()) {
            total += split[split.len() / 2];
        }
    }

    Ok(total)
}

fn calculate_part_two(
    pairs: Lines<BufReader<File>>,
    lines: Lines<BufReader<File>>,
) -> Result<usize> {
    let mut total = 0;
    let mut pairs_vec = Vec::new();

    for pair in pairs.map_while(Result::ok) {
        let values = pair.split_once('|').unwrap();
        let x = values.0.parse::<usize>().unwrap();
        let y = values.1.parse::<usize>().unwrap();
        pairs_vec.push((x, y));
    }

    for line in lines.map_while(Result::ok) {
        let split = line
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if !is_correct_order(&pairs_vec, split.clone()) {
            let corrected = correct_order(&pairs_vec, &split, &create_pages_map(&split));
            total += corrected[corrected.len() / 2];
        }
    }

    Ok(total)
}

fn create_pages_map(pages: &[usize]) -> HashMap<&usize, usize> {
    let mut page_number_to_index_map = HashMap::new();

    for (index, page) in pages.iter().enumerate() {
        page_number_to_index_map.insert(page, index);
    }

    page_number_to_index_map
}

fn is_correct_order(graph: &Vec<(usize, usize)>, pages: Vec<usize>) -> bool {
    let mut valid = true;
    for (first, second) in graph {
        let first_pos = pages.iter().position(|x| x == first);
        let second_pos = pages.iter().position(|x| x == second);
        match (first_pos, second_pos) {
            (Some(first_index), Some(second_index)) => {
                if first_index > second_index {
                    valid = false;
                    break;
                }
            }
            _ => continue,
        }
    }

    valid
}

fn correct_order(
    pairs: &Vec<(usize, usize)>,
    pages: &Vec<usize>,
    pages_map: &HashMap<&usize, usize>,
) -> Vec<usize> {
    let mut page_number_to_index_map = pages_map.clone();
    let mut sorted = pages.to_owned();

    for (lhs, rhs) in pairs {
        if !page_number_to_index_map.contains_key(lhs) {
            continue;
        }
        if !page_number_to_index_map.contains_key(rhs) {
            continue;
        }
        let lhs_idx = *page_number_to_index_map.get(lhs).unwrap();
        let rhs_idx = *page_number_to_index_map.get(rhs).unwrap();

        if lhs_idx > rhs_idx {
            sorted.swap(lhs_idx, rhs_idx);
            page_number_to_index_map.insert(lhs, rhs_idx);
            page_number_to_index_map.insert(rhs, lhs_idx);
        }
    }

    if !is_correct_order(pairs, pages.to_owned()) {
        return correct_order(pairs, &sorted, &page_number_to_index_map);
    }

    sorted
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
        let pairs_file_path = base.join("pairs.txt");
        let mut pairs_file = File::create(&pairs_file_path).unwrap();
        let pairs_data = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13"#;
        pairs_file.write_all(pairs_data.as_bytes()).unwrap();
        let pairs = read_lines(pairs_file_path).unwrap();

        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_one(pairs, lines).unwrap();

        assert_eq!(result, 143);
    }

    #[test]
    fn test_part_two() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let pairs_file_path = base.join("pairs.txt");
        let mut pairs_file = File::create(&pairs_file_path).unwrap();
        let pairs_data = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13"#;
        pairs_file.write_all(pairs_data.as_bytes()).unwrap();
        let pairs = read_lines(pairs_file_path).unwrap();

        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_two(pairs, lines).unwrap();

        assert_eq!(result, 123);
    }
}
