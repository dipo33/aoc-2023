use std::fs;
use std::path::Path;
use crate::parser;

pub fn execute<P: AsRef<Path>, F: Fn(&mut Vec<i64>) -> i64>(path: P, name: &str, print: bool, process_row: F) -> i64 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let rows = parser::parse(&contents)
        .expect("Should have been able to parse the file");

    let mut result: i64 = 0;
    for mut row in rows {
        result += process_row(&mut row);
    }

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}