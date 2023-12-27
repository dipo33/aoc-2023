use crate::{part1, part2};

#[test]
fn day09_part1_example() {
    assert_eq!(
        part1::execute("inputs/example1.txt", "", false),
        u32::default(),
        "Day 09 - Part 01 Example Test",
    );
}

#[test]
fn day09_part2_example() {
    assert_eq!(
        part2::execute("inputs/example2.txt", "", false),
        u32::default(),
        "Day 09 - Part 02 Example Test",
    );
}
