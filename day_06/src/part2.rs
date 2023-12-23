use std::fs;
use std::path::Path;

use crate::entity::{Race};
use crate::parser;

fn merge_numbers(a: u64, b: u64) -> u64 {
    let power = b.checked_ilog10().unwrap_or(0) + 1;
    a * 10u64.pow(power) + b
}

#[allow(unused_variables)]
pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let races = parser::parse(&contents)
        .expect("Should have been able to parse the file");

    let race = races.iter()
        .fold(Race { time: 0, record: 0 }, |acc, race| {
            Race {
                record: merge_numbers(acc.record, race.record),
                time: merge_numbers(acc.time, race.time),
            }
        });

    let result = race.winning_times_count();
    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
