use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::entity::{Blueprint, Item, neighbours, Position};
use crate::parser;

fn find_parts_around(pos: Position, part_ids: &mut HashSet<Position>, blueprint: &Blueprint) {
    for n_pos in neighbours(pos) {
        if let (pos, Some(Item::PartLabel(_))) = blueprint.get_item_at(n_pos) {
            part_ids.insert(pos);
        }
    }
}

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let (_, blueprint) = parser::parse(&contents)
        .expect("Should have been able to parse the file");

    let mut part_ids: HashSet<Position> = HashSet::new();
    for pos in blueprint.pos_iter() {
        if let (_, Some(Item::Symbol(_))) = blueprint.get_item_at(pos) {
            find_parts_around(pos, &mut part_ids, &blueprint);
        }
    }

    let mut result: u32 = 0;
    for id in part_ids {
        if let Some(Item::PartLabel(x)) = blueprint.get_item_at(id).1 {
            result += x;
        }
    }

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
