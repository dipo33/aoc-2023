use std::time::{Duration, Instant};

mod part1;
mod part2;


pub fn solve(first: bool, second: bool) {
    println!("\n-------- [[ Task 02 ]] --------");
    if first {
        println!("Part 1:");
        part1::execute("task_02/inputs/example1.txt", "    Example", true);
        part1::execute("task_02/inputs/input.txt", "   ", true);
    }

    if second {
        if first {
            println!();
        }
        println!("Part 2:");
        part2::execute("task_02/inputs/example2.txt", "    Example", true);
        part2::execute("task_02/inputs/input.txt", "   ", true);
    }
}

pub fn measure(attempts: u32) -> (Duration, Duration) {
    println!("\n-------- [[ Task 02 ]] --------");

    let beginning = Instant::now();
    for _ in 0..attempts {
        part1::execute("task_02/inputs/input.txt", "Part 1", false);
    }
    let average_1 = beginning.elapsed() / attempts;
    println!("Part 1 Average Time: {:?}", average_1);


    let beginning = Instant::now();
    for _ in 0..attempts {
        part2::execute("task_02/inputs/input.txt", "Part 2", false);
    }
    let average_2 = beginning.elapsed() / attempts;
    println!("Part 2 Average Time: {:?}", average_2);
    println!("Attempts: {}", attempts);

    (average_1, average_2)
}
