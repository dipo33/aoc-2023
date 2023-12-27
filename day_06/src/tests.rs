use crate::{part1, part2};

#[test]
fn day06_part1_example() {
    assert_eq!(
        part1::execute("inputs/example1.txt", "", false),
        288,
        "Day 06 - Part 01 Example Test",
    );
}

#[test]
fn day06_part2_example() {
    assert_eq!(
        part2::execute("inputs/example2.txt", "", false),
        71503,
        "Day 06 - Part 02 Example Test",
    );
}
