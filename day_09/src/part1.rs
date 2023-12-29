use std::path::Path;

use crate::common;

fn process_row(row: &mut Vec<i64>) -> i64 {
    let mut rightmost_idx = row.len() - 1;
    while rightmost_idx > 0 {
        for i in 0..rightmost_idx {
            let a = row[i];
            let b = row[i + 1];

            row[i] = b - a;
        }
        rightmost_idx -= 1;
    }

    row.iter().sum()
}

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> i64 {
    common::execute(path, name, print, process_row)
}
