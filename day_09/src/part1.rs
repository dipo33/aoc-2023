use std::path::Path;

use crate::common;

fn process_row(row: &mut Vec<i64>) -> i64 {
    let mut end = row.len() - 1;
    while end >= 1 {
        for i in 0..end {
            // TODO: Make more efficient by skipping when all are zero
            let a = row[i];
            let b = row[i + 1];

            row[i] = b - a;
        }
        end -= 1;
    }

    row.iter().sum()
}

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> i64 {
    common::execute(path, name, print, process_row)
}
