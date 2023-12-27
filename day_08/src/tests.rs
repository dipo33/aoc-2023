use crate::{part1, part2};

#[test]
fn day08_part1_example_a() {
    assert_eq!(
        part1::execute("inputs/example1_a.txt", "", false),
        2,
        "Day 08 - Part 01 Example A Test",
    );
}

#[test]
fn day08_part1_example_b() {
    assert_eq!(
        part1::execute("inputs/example1_b.txt", "", false),
        6,
        "Day 08 - Part 01 Example B Test",
    );
}

#[test]
fn day08_part2_example() {
    assert_eq!(
        part2::execute("inputs/example2.txt", "", false),
        6,
        "Day 08 - Part 02 Example Test",
    );
}
