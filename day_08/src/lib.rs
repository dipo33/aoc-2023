use std::time::{Duration, Instant};

pub mod part1;
pub mod part2;
pub mod entity;
pub mod parser;
#[cfg(test)]
mod tests;


pub fn test_first(result_a: u32, result_b: u32) {
    assert_eq!(
        part1::execute("day_08/inputs/example1_a.txt", "", false),
        result_a,
        "Day 08 - Part 01 Example A Test",
    );

    assert_eq!(
        part1::execute("day_08/inputs/example1_b.txt", "", false),
        result_b,
        "Day 08 - Part 01 Example B Test",
    );
}

pub fn test_second(result: usize) {
    assert_eq!(
        part2::execute("day_08/inputs/example2.txt", "", false),
        result,
        "Day 08 - Part 02 Example Test",
    );
}

pub fn solve(first: bool, second: bool) {
    println!("\n-------- [[ Day 08 ]] --------");
    if first {
        println!("Part 1:");
        part1::execute("day_08/inputs/example1_a.txt", "    Example A", true);
        part1::execute("day_08/inputs/example1_b.txt", "    Example B", true);
        part1::execute("day_08/inputs/input.txt", "   ", true);
    }

    if second {
        if first {
            println!();
        }
        println!("Part 2:");
        part2::execute("day_08/inputs/example2.txt", "    Example", true);
        part2::execute("day_08/inputs/input.txt", "   ", true);
    }
}

pub fn measure(attempts: u32) -> (Duration, Duration) {
    println!("\n-------- [[ Day 08 ]] --------");

    let beginning = Instant::now();
    for _ in 0..attempts {
        part1::execute("day_08/inputs/input.txt", "", false);
    }
    let average_1 = beginning.elapsed() / attempts;
    println!("Part 1 Average Time: {:?}", average_1);


    let beginning = Instant::now();
    for _ in 0..attempts {
        part2::execute("day_08/inputs/input.txt", "", false);
    }
    let average_2 = beginning.elapsed() / attempts;
    println!("Part 2 Average Time: {:?}", average_2);
    println!("Attempts: {}", attempts);

    (average_1, average_2)
}
