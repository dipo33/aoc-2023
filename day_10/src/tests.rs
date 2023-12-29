use crate::{part1, part2};

#[test]
fn day10_part1_a_example() {
    assert_eq!(
        part1::execute("inputs/example1_a.txt", "", false),
        4,
        "Day 10 - Part 01 Example A Test",
    );
}

#[test]
fn day10_part1_b_example() {
    assert_eq!(
        part1::execute("inputs/example1_b.txt", "", false),
        8,
        "Day 10 - Part 01 Example B Test",
    );
}

#[test]
fn day10_part2_example() {
    assert_eq!(
        part2::execute("inputs/example2.txt", "", false),
        u32::default(),
        "Day 10 - Part 02 Example Test",
    );
}
