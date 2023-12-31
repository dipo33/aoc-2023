pub mod part1;
pub mod part2;
#[cfg(test)]
mod tests;
pub mod parser;
mod common;

pub fn solve(first: bool, second: bool) {
    println!("\n-------- [[ Day 09 ]] --------");
    if first {
        println!("Part 1:");
        part1::execute("day_09/inputs/example1.txt", "    Example", true);
        part1::execute("day_09/inputs/input.txt", "   ", true);
    }

    if second {
        if first {
            println!();
        }
        println!("Part 2:");
        part2::execute("day_09/inputs/example2.txt", "    Example", true);
        part2::execute("day_09/inputs/input.txt", "   ", true);
    }
}
