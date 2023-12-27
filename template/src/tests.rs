use crate::{part1, part2};

#[test]
fn day%%FORMATTED_DAY%%_part1_example() {
    assert_eq!(
        part1::execute("inputs/example1.txt", "", false),
        %%RESULT_TYPE%%::default(),
        "Day %%FORMATTED_DAY%% - Part 01 Example Test",
    );
}

#[test]
fn day%%FORMATTED_DAY%%_part2_example() {
    assert_eq!(
        part2::execute("inputs/example2.txt", "", false),
        %%RESULT_TYPE%%::default(),
        "Day %%FORMATTED_DAY%% - Part 02 Example Test",
    );
}
