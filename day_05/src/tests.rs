use crate::{part1, part2};

#[test]
fn day05_part1_example() {
    assert_eq!(
        part1::execute("inputs/example1.txt", "", false),
        35,
        "Day 05 - Part 01 Example Test",
    );
}

#[test]
fn day05_part2_example() {
    assert_eq!(
        part2::execute("inputs/example2.txt", "", false),
        46,
        "Day 05 - Part 02 Example Test",
    );
}
