use std::fs;
use std::path::Path;

use crate::entity::{Direction, Graph};
use crate::parser::parse;

#[allow(unused_variables)]
pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let parsed = parse(&contents)
        .expect("Should have been able to parse the file");

    let dest_node = Graph::label_to_node_id("ZZZ");
    let mut current_node = Graph::label_to_node_id("AAA");
    let mut steps = 0;
    while current_node != dest_node {
        let (dest_a, dest_b) = *parsed.graph.edges.get(&current_node).unwrap();
        let direction = parsed.directions[steps % parsed.directions.len()];
        current_node = match direction {
            Direction::Left => dest_a,
            Direction::Right => dest_b,
        };
        steps += 1;
    }

    if print {
        println!("{} Result: {}", name, steps);
    }

    steps as u32
}
