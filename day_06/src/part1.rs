use std::fs;
use std::path::Path;
use crate::entity::Race;

use crate::parser;

#[allow(unused_variables)]
pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let races = parser::parse(&contents)
        .expect("Should have been able to parse the file");

    let result = races.iter()
        .map(Race::winning_times_count)
        .product::<u32>();

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
