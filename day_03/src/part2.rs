use std::collections::HashSet;
use std::fs;
use std::path::Path;
use crate::entity::{Blueprint, Item, neighbours, Position};

use crate::parser;

fn get_gear_ratio(pos: Position, blueprint: &Blueprint) -> u32 {
    let mut part_ids: HashSet<Position> = HashSet::with_capacity(3);
    for n_pos in neighbours(pos) {
        if let (pos, Some(Item::PartLabel(_))) = blueprint.get_item_at(n_pos) {
            part_ids.insert(pos);
        }
        if part_ids.len() > 2 {
            return 0;
        }
    }


    if part_ids.len() == 2 {
        return part_ids.into_iter().map(|pos| {
            if let Some(Item::PartLabel(x)) = blueprint.get_item_at(pos).1 {
                x
            } else {
                0
            }
        }).product();
    }

    return 0;
}

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let (_, blueprint) = parser::parse(&contents)
        .expect("Should have been able to parse the file");

    let mut result: u32 = 0;
    for pos in blueprint.pos_iter() {
        if let (_, Some(Item::Symbol('*'))) = blueprint.get_item_at(pos) {
            result += get_gear_ratio(pos, &blueprint);
        }
    }

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
