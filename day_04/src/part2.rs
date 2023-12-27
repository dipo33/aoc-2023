use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::entity::Card;
use crate::parser;

fn correct_guesses(card: &Card) -> u32 {
    let mut correct = 0;
    for guess in card.guesses.iter() {
        if card.winning.contains(guess) {
            correct += 1;
        }
    }

    correct
}

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let cards = parser::parse(&contents)
        .expect("Should have been able to parse the file");

    let mut copies: HashMap<u32, u32> = HashMap::with_capacity(cards.len());
    let mut result: u32 = 0;
    for card in cards.iter() {
        let card_count = *copies.get(&card.id).unwrap_or(&0) + 1;
        result += card_count;

        let correct_guesses = correct_guesses(card);
        for copy_id in card.id + 1..=card.id + correct_guesses {
            let copy_count = copies.entry(copy_id).or_insert(0);
            *copy_count += card_count;
        }
    }

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
