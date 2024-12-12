use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufReader, Lines},
};

use anyhow::Result;

use crate::{
    utils::{build_data_file_path, build_twod_vec, is_in_bounds, read_lines},
    Day,
};

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn puzzle1() {
    let file_path = build_data_file_path(&Day::Day12, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_one(lines).unwrap();

    println!("{result}");
}

pub fn puzzle2() {
    let file_path = build_data_file_path(&Day::Day12, "data.txt").unwrap();
    let lines = read_lines(file_path).unwrap();
    let result = calculate_part_two(lines).unwrap();

    println!("{result}");
}

fn calculate_part_one(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;
    let grid = build_twod_vec(lines).unwrap();
    let mut queue = VecDeque::new();
    let mut checked = HashSet::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if checked.contains(&(row, col)) {
                continue;
            }
            queue.push_back((row, col));
            let mut area = 0;
            let mut perim = 0;
            let mut perim_tracker: HashMap<(isize, isize), HashSet<(isize, isize)>> =
                HashMap::new();
            while !queue.is_empty() {
                let (row2, col2) = queue.pop_front().unwrap();
                if checked.contains(&(row2, col2)) {
                    continue;
                }
                checked.insert((row2, col2));
                area += 1;
                for (dr, dc) in DIRECTIONS {
                    let next_row = row2 as isize + dr;
                    let next_col = col2 as isize + dc;
                    if is_in_bounds(&grid, next_row, next_col)
                        && grid[next_row as usize][next_col as usize] == grid[row2][col2]
                    {
                        queue.push_back((next_row as usize, next_col as usize));
                    } else {
                        perim += 1;
                        perim_tracker
                            .entry((dr, dc))
                            .and_modify(|x| {
                                x.insert((dr, dc));
                            })
                            .or_insert_with(|| {
                                let mut set = HashSet::new();
                                set.insert((dr, dc));
                                set
                            });
                    }
                }
            }

            total += area * perim;
        }
    }

    Ok(total)
}

fn calculate_part_two(lines: Lines<BufReader<File>>) -> Result<usize> {
    let mut total = 0;
    let grid = build_twod_vec(lines).unwrap();
    let mut queue = VecDeque::new();
    let mut checked = HashSet::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if checked.contains(&(row, col)) {
                continue;
            }
            queue.push_back((row, col));
            let mut area = 0;
            let mut perim_tracker: HashMap<(isize, isize), HashSet<(isize, isize)>> =
                HashMap::new();
            while !queue.is_empty() {
                let (row2, col2) = queue.pop_front().unwrap();
                if checked.contains(&(row2, col2)) {
                    continue;
                }
                checked.insert((row2, col2));
                area += 1;
                for (dr, dc) in DIRECTIONS {
                    let next_row = row2 as isize + dr;
                    let next_col = col2 as isize + dc;
                    if is_in_bounds(&grid, next_row, next_col)
                        && grid[next_row as usize][next_col as usize] == grid[row2][col2]
                    {
                        queue.push_back((next_row as usize, next_col as usize));
                    } else {
                        perim_tracker
                            .entry((dr, dc))
                            .and_modify(|x| {
                                x.insert((next_row, next_col));
                            })
                            .or_insert_with(|| {
                                let mut set = HashSet::new();
                                set.insert((next_row, next_col));
                                set
                            });
                    }
                }
            }

            let mut sides = 0;
            for (_, v) in perim_tracker.iter() {
                let mut checked_perim: HashSet<(isize, isize)> = HashSet::new();
                for &(prev_row, prev_col) in v {
                    if !checked_perim.contains(&(prev_row, prev_col)) {
                        sides += 1;
                        let mut perim_queue = VecDeque::new();
                        perim_queue.push_back((prev_row, prev_col));
                        while !perim_queue.is_empty() {
                            let (new_row, new_col) = perim_queue.pop_front().unwrap();
                            if checked_perim.contains(&(new_row, new_col)) {
                                continue;
                            }
                            checked_perim.insert((new_row, new_col));
                            for (dr, dc) in DIRECTIONS {
                                let nr = new_row + dr;
                                let nc = new_col + dc;
                                if v.contains(&(nr, nc)) {
                                    perim_queue.push_back((nr, nc));
                                }
                            }
                        }
                    }
                }
            }
            println!("{:?}", perim_tracker);
            total += area * sides;
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
        let data = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_one(lines).unwrap();

        assert_eq!(result, 1930);
    }

    #[test]
    fn test_part_two() {
        let base = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&base).unwrap();
        let file_path = base.join("data.txt");
        let mut file = File::create(&file_path).unwrap();
        let data = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        file.write_all(data.as_bytes()).unwrap();
        let lines = read_lines(file_path).unwrap();
        let result = calculate_part_two(lines).unwrap();

        assert_eq!(result, 1206);
    }
}
