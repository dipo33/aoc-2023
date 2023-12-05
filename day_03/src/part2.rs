use std::fs;
use std::path::Path;
use crate::entity::SchematicItem;
use crate::parser;

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let (_, schematic) = parser::parse(&contents)
        .expect("Should have been able to parse the file");

    let result: u32 = schematic.engine_parts()
        .iter()
        .map(|item| match item {
            SchematicItem::PartLabel { part_number, .. } => part_number,
            _ => panic!("Should have been a part label"),
        })
        .sum::<u32>();

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
