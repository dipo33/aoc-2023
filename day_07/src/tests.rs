use crate::{part1, part2};

#[test]
fn day07_part1_example() {
    assert_eq!(
        part1::execute("inputs/example1.txt", "", false),
        6440,
        "Day 07 - Part 01 Example Test",
    );
}

#[test]
fn day07_part2_example() {
    assert_eq!(
        part2::execute("inputs/example2.txt", "", false),
        5905,
        "Day 07 - Part 02 Example Test",
    );
}
