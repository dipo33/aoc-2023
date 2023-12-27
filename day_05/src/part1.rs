use std::fs;
use std::path::Path;
use crate::parser;

#[allow(unused_variables)]
pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let almanac = parser::parse(&contents)
        .expect("Should have been able to parse the input file");

    let mut lowest_location: Option<u32> = None;
    for mut current in almanac.seeds {
        for interval_tree in almanac.interval_trees.iter() {
            current = interval_tree.translate(current);
        }

        if lowest_location.is_none() || current < lowest_location.unwrap() {
            lowest_location = Some(current);
        }
    }

    if print {
        println!("{} Result: {}", name, lowest_location.unwrap());
    }

    lowest_location.unwrap()
}
