use std::fs;
use std::path::Path;

use crate::entity::{Blueprint, BlueprintItem, Index};
use crate::parser;

fn get_gear_ratio(idx: Index, blueprint: &Blueprint) -> u32 {
    let mut p1: Option<(Index, u32)> = None;
    let mut p2: Option<(Index, u32)> = None;

    for idx_n in blueprint.neighbours(idx) {
        if let BlueprintItem::PartLabel(label, idx_p) = blueprint.get(idx_n) {
            if p1.is_none() {
                p1 = Some((idx_p, label));
            } else if p2.is_none() {
                if p1.unwrap().0 != idx_p {
                    p2 = Some((idx_p, label));
                }
            } else if (p1.unwrap().0 != idx_p) && (p2.unwrap().0 != idx_p) {
                return 0;
            }
        }
    }

    return p1.unwrap_or_default().1 * p2.unwrap_or_default().1;
}

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let (_, blueprint) = parser::parse(&contents)
        .expect("Should have been able to parse the file");

    let mut result: u32 = 0;
    for idx in blueprint.indices() {
        if let BlueprintItem::Symbol('*') = blueprint.get(idx) {
            result += get_gear_ratio(idx, &blueprint);
        }
    }

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
