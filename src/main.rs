use std::{path::Path, fs::{self, File}, io::{BufReader, BufRead}};

fn main() {
    println!("Total lines: {}", files_recursively(Path::new("../BlueSoulsII/"), 0));
}

fn files_recursively(path: &Path, mut lines: i64) -> i64 {
    let mut total_lines = lines;
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if path.to_str().unwrap().contains("target") {
                continue;
            }

            if path.to_str().unwrap().contains("node_modules") {
                continue;
            }

            if path.to_str().unwrap().contains(".idea") {
                continue;
            }

            if path.to_str().unwrap().contains("out") {
                continue;
            }

            if path.to_str().unwrap().contains(".git") {
                continue;
            }

            total_lines = files_recursively(&path, total_lines);
        } else {
            let file = File::open(&path).unwrap();
            let reader = BufReader::new(file);
            total_lines += reader.lines().count() as i64;
        }
    }
    total_lines
}