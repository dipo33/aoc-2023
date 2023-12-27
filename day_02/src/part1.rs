use std::fs;
use std::path::Path;

use crate::entity::Color;
use crate::parser;

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let games = parser::parse(contents.as_str())
        .unwrap();

    let result = games.iter()
        .filter(|game| game.get_cube_amount(Color::RED) <= 12)
        .filter(|game| game.get_cube_amount(Color::GREEN) <= 13)
        .filter(|game| game.get_cube_amount(Color::BLUE) <= 14)
        .map(|game| game.id as u32)
        .sum::<u32>();

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
