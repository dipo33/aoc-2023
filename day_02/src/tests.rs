use crate::{part1, part2};

#[test]
fn day02_part1_example() {
    assert_eq!(
        part1::execute("inputs/example1.txt", "", false),
        8,
        "Day 02 - Part 01 Example Test",
    );
}

#[test]
fn day02_part2_example() {
    assert_eq!(
        part2::execute("inputs/example2.txt", "", false),
        2286,
        "Day 02 - Part 02 Example Test",
    );
}
