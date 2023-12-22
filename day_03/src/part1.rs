use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::entity::{Blueprint, BlueprintItem, Index};
use crate::parser;

fn find_parts_around(idx: Index, part_indices: &mut HashSet<Index>, blueprint: &Blueprint) -> u32 {
    let mut result: u32 = 0;
    for idx_n in blueprint.neighbours(idx) {
        if let BlueprintItem::PartLabel(label, idx_p) = blueprint.get(idx_n) {
            if part_indices.insert(idx_p) {
                result += label;
            }
        }
    }

    result
}

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let (_, blueprint) = parser::parse(&contents)
        .expect("Should have been able to parse the file");

    let mut part_indices: HashSet<Index> = HashSet::new();
    let mut result: u32 = 0;
    for idx in blueprint.indices() {
        if let BlueprintItem::Symbol(_) = blueprint.get(idx) {
            result += find_parts_around(idx, &mut part_indices, &blueprint);
        }
    }

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
