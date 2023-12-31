use std::path::Path;

use crate::common;

fn process_row(row: &mut Vec<i64>) -> i64 {
    let mut leftmost_idx = 0;
    while leftmost_idx < row.len() - 1 {
        for i in (leftmost_idx + 1..row.len()).rev() {
            let a = row[i];
            let b = row[i - 1];

            row[i] = a - b;
        }
        leftmost_idx += 1;
    }

    row.iter().rev().fold(0, |acc, x| x - acc)
}

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> i64 {
    common::execute(path, name, print, process_row)
}
