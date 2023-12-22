use std::fs;
use std::path::Path;

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let result: u32 = contents
        .split_whitespace()
        .map(|str: &str| (
            str.chars().find(char::is_ascii_digit).unwrap().to_digit(10).unwrap(),
            str.chars().rfind(char::is_ascii_digit).unwrap().to_digit(10).unwrap()
        ))
        .map(|(first, last)| first * 10 + last)
        .sum();

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
