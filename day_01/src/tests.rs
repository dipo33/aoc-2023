use crate::{part1, part2};

#[test]
fn day01_part1_example() {
    assert_eq!(
        part1::execute("inputs/example1.txt", "", false),
        142,
        "Day 01 - Part 01 Example Test",
    );
}

#[test]
fn day01_part2_example() {
    assert_eq!(
        part2::execute("inputs/example2.txt", "", false),
        281,
        "Day 01 - Part 02 Example Test",
    );
}
