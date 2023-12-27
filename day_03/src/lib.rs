pub mod part1;
pub mod part2;
mod entity;
pub mod parser;
#[cfg(test)]
mod tests;

pub fn solve(first: bool, second: bool) {
    println!("\n-------- [[ Day 03 ]] --------");
    if first {
        println!("Part 1:");
        part1::execute("day_03/inputs/example1.txt", "    Example", true);
        part1::execute("day_03/inputs/input.txt", "   ", true);
    }

    if second {
        if first {
            println!();
        }
        println!("Part 2:");
        part2::execute("day_03/inputs/example2.txt", "    Example", true);
        part2::execute("day_03/inputs/input.txt", "   ", true);
    }
}
