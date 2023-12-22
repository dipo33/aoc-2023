use std::time::{Duration, Instant};

mod part1;
mod part2;


pub fn test_first(result: u32) {
    assert_eq!(
        part1::execute("day_04/inputs/example1.txt", "", false),
        result,
        "Day 04 - Part 01 Example Test",
    );
}

pub fn test_second(result: u32) {
    assert_eq!(
        part2::execute("day_04/inputs/example2.txt", "", false),
        result,
        "Day 04 - Part 02 Example Test",
    );
}

pub fn solve(first: bool, second: bool) {
    println!("\n-------- [[ Day 04 ]] --------");
    if first {
        println!("Part 1:");
        part1::execute("day_04/inputs/example1.txt", "    Example", true);
        part1::execute("day_04/inputs/input.txt", "   ", true);
    }

    if second {
        if first {
            println!();
        }
        println!("Part 2:");
        part2::execute("day_04/inputs/example2.txt", "    Example", true);
        part2::execute("day_04/inputs/input.txt", "   ", true);
    }
}

pub fn measure(attempts: u32) -> (Duration, Duration) {
    println!("\n-------- [[ Day 04 ]] --------");

    let beginning = Instant::now();
    for _ in 0..attempts {
        part1::execute("day_04/inputs/input.txt", "", false);
    }
    let average_1 = beginning.elapsed() / attempts;
    println!("Part 1 Average Time: {:?}", average_1);


    let beginning = Instant::now();
    for _ in 0..attempts {
        part2::execute("day_04/inputs/input.txt", "", false);
    }
    let average_2 = beginning.elapsed() / attempts;
    println!("Part 2 Average Time: {:?}", average_2);
    println!("Attempts: {}", attempts);

    (average_1, average_2)
}
