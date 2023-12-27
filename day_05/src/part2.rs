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
    for i in 0..almanac.seeds.len() / 2 {
        let (current, length) = (almanac.seeds[i * 2], almanac.seeds[i * 2 + 1]);
        let mut intervals = vec![(current, current + (length - 1))];
        for interval_tree in almanac.interval_trees.iter() {
            let mut new_intervals = Vec::new();
            for (start, end) in intervals {
                new_intervals.extend(interval_tree.overlaps(start, end));
            }
            intervals = new_intervals;
        }

        for interval in intervals {
            if lowest_location.is_none() || interval.0 < lowest_location.unwrap() {
                lowest_location = Some(interval.0);
            }
        }
    }

    if print {
        println!("{} Result: {}", name, lowest_location.unwrap());
    }

    lowest_location.unwrap()
}
