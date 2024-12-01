use std::{
    env::current_dir,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::{Path, PathBuf},
};

use anyhow::Result;

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn build_data_file_path(file_path: &str) -> Result<PathBuf> {
    let mut built_path = current_dir().unwrap();
    built_path.push(file_path);

    Ok(built_path)
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
}
