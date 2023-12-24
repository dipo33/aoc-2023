use std::fs;
use std::path::Path;

use crate::entity::Bidder;
use crate::parser;

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let mut bidders: Vec<Bidder> = parser::parse(&contents, true)
        .expect("Should have been able to parse the file");
    bidders.sort_by(|a, b| b.hand.cmp(&a.hand));
    bidders.reverse();


    let mut rank: u32 = 1;
    let mut total_winnings: u32 = 0;
    for bidder in &bidders {
        total_winnings += bidder.bid * rank;
        rank += 1;
    }

    if print {
        println!("{} Result: {}", name, total_winnings);
    }

    total_winnings
}
