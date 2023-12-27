use crate::{part1, part2};

#[test]
fn day03_part1_example() {
    assert_eq!(
        part1::execute("inputs/example1.txt", "", false),
        4361,
        "Day 03 - Part 01 Example Test",
    );
}

#[test]
fn day03_part2_example() {
    assert_eq!(
        part2::execute("inputs/example2.txt", "", false),
        467835,
        "Day 03 - Part 02 Example Test",
    );
}
