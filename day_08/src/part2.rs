use std::fs;
use std::path::Path;

use num::Integer;

use crate::entity::{Direction, LabeledMap, NodeId};
use crate::parser;

const ENDS_WITH_A: u32 = 'A' as u32 * 256u32.pow(2);
const ENDS_WITH_Z: u32 = 'Z' as u32 * 256u32.pow(2);
const LAST_CHARACTER: u32 = 0xFF0000;

#[allow(unused_variables)]
pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> usize {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let map: LabeledMap = parser::parse(&contents)
        .expect("Should have been able to parse the file");

    let start_nodes: Vec<NodeId> = map.graph.edges.keys()
        .map(|node| *node)
        .filter(|node| node & LAST_CHARACTER == ENDS_WITH_A)
        .collect();

    let mut steps_per_node: Vec<usize> = Vec::with_capacity(start_nodes.len());
    for mut current_node in start_nodes {
        let mut steps = 0;
        loop {
            let (dest_a, dest_b) = *map.graph.edges.get(&current_node).unwrap();
            let direction = map.directions[steps % map.directions.len()];
            current_node = match direction {
                Direction::Left => dest_a,
                Direction::Right => dest_b,
            };
            steps += 1;

            if current_node & LAST_CHARACTER == ENDS_WITH_Z {
                steps_per_node.push(steps);
                break;
            }
        }
    }

    let result = steps_per_node.iter().fold(1, |acc, steps| acc.lcm(steps));
    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
