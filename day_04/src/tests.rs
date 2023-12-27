use crate::{part1, part2};

#[test]
fn day04_part1_example() {
    assert_eq!(
        part1::execute("inputs/example1.txt", "", false),
        13,
        "Day 04 - Part 01 Example Test",
    );
}

#[test]
fn day04_part2_example() {
    assert_eq!(
        part2::execute("inputs/example2.txt", "", false),
        30,
        "Day 04 - Part 02 Example Test",
    );
}
