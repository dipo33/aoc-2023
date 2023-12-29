use std::path::Path;

use crate::common;

fn process_row(row: &mut Vec<i64>) -> i64 {
    let mut end = 0;
    while end < row.len() - 1 {
        for i in (end + 1..row.len()).rev() {
            // TODO: Make more efficient by skipping when all are zero
            let a = row[i];
            let b = row[i - 1];

            row[i] = a - b;
        }
        end += 1;
    }

    row.iter().rev().fold(0, |acc, x| -acc + x)
}

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> i64 {
    common::execute(path, name, print, process_row)
}
