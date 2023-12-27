pub mod part1;
pub mod part2;
pub mod part2_trie;
#[cfg(test)]
mod tests;

pub fn solve(first: bool, second: bool) {
    println!("\n-------- [[ Day 01 ]] --------");
    if first {
        println!("Part 1:");
        part1::execute("day_01/inputs/example1.txt", "    Example", true);
        part1::execute("day_01/inputs/input.txt", "   ", true);
    }

    if second {
        if first {
            println!();
        }
        println!("Part 2:");
        part2::execute("day_01/inputs/example2.txt", "    Example", true);
        part2::execute("day_01/inputs/input.txt", "   ", true);

        println!("\nPart 2 with Trie:");
        part2_trie::execute("day_01/inputs/example2.txt", "    Example", true);
        part2_trie::execute("day_01/inputs/input.txt", "   ", true);
    }
}
