pub mod part1;
pub mod part2;
#[cfg(test)]
mod tests;

pub fn solve(first: bool, second: bool) {
    println!("\n-------- [[ Day 10 ]] --------");
    if first {
        println!("Part 1:");
        part1::execute("day_10/inputs/example1_a.txt", "    Example A", true);
        part1::execute("day_10/inputs/example1_b.txt", "    Example B", true);
        part1::execute("day_10/inputs/input.txt", "   ", true);
    }

    if second {
        if first {
            println!();
        }
        println!("Part 2:");
        part2::execute("day_10/inputs/example2.txt", "    Example", true);
        part2::execute("day_10/inputs/input.txt", "   ", true);
    }
}
