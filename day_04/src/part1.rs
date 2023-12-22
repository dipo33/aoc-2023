use std::fs;
use std::path::Path;

use crate::entity::Card;
use crate::parser;

fn points(card: &Card) -> u32 {
    let mut points = 0;
    for guess in card.guesses.iter() {
        if card.winning.contains(guess) {
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }
    }

    points
}

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let (_, cards) = parser::parse(&contents)
        .expect("Should have been able to parse the file");
    let result: u32 = cards.iter()
        .map(points)
        .sum();

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
