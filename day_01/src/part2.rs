use std::collections::HashMap;
use std::fs;
use std::path::Path;

use lazy_static::lazy_static;

lazy_static! {
    static ref DIGIT_LOOKUP: HashMap<&'static str, char> = {
        let mut lookup = HashMap::new();
        lookup.insert("one", '1');
        lookup.insert("two", '2');
        lookup.insert("three", '3');
        lookup.insert("four", '4');
        lookup.insert("five", '5');
        lookup.insert("six", '6');
        lookup.insert("seven", '7');
        lookup.insert("eight", '8');
        lookup.insert("nine", '9');

        lookup
    };
}

trait SubstringIter {
    fn substrings(&self) -> Box<dyn Iterator<Item=&str> + '_>;
    fn rsubstrings(&self) -> Box<dyn Iterator<Item=&str> + '_>;
}

impl SubstringIter for &str {
    fn substrings(&self) -> Box<dyn Iterator<Item=&str> + '_> {
        let indices = self.char_indices().map(|(i, _)| i);
        Box::new(indices.map(move |i| &self[i..]))
    }

    fn rsubstrings(&self) -> Box<dyn Iterator<Item=&str> + '_> {
        let indices = self.char_indices().map(|(i, _)| i).rev();
        Box::new(indices.map(move |i| &self[i..]))
    }
}

fn get_digit(str: &str) -> Option<char> {
    for key in DIGIT_LOOKUP.keys() {
        if str.starts_with(key) {
            return DIGIT_LOOKUP.get(key).cloned();
        }
    }

    str.chars().next().filter(char::is_ascii_digit)
}

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");


    let result: u32 = contents
        .split_whitespace()
        .map(|str| (str.substrings().map(get_digit).find(Option::is_some), str.rsubstrings().map(get_digit).find(Option::is_some)))
        .map(|(first, last)| (first.unwrap().unwrap(), last.unwrap().unwrap()))
        .map(|(first, last)| (first.to_digit(10).unwrap(), last.to_digit(10).unwrap()))
        .map(|(first, last)| first * 10 + last)
        .sum();

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
