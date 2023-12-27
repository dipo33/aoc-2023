use std::fs;
use std::path::Path;

use crate::entity::Color;
use crate::parser;

#[allow(unused_variables)]
pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let games = parser::parse(contents.as_str())
        .unwrap();

    let result: u32 = games.iter()
        .map(|game| {
            let red: u32 = game.get_cube_amount(Color::RED) as u32;
            let green: u32 = game.get_cube_amount(Color::GREEN) as u32;
            let blue: u32 = game.get_cube_amount(Color::BLUE) as u32;

            red * green * blue
        })
        .sum::<u32>();

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
