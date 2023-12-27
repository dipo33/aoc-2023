pub mod part1;
pub mod part2;
#[cfg(test)]
mod tests;

pub fn solve(first: bool, second: bool) {
    println!("\n-------- [[ %%DISPLAY_NAME%% ]] --------");
    if first {
        println!("Part 1:");
        part1::execute("%%PACKAGE_NAME%%/inputs/example1.txt", "    Example", true);
        part1::execute("%%PACKAGE_NAME%%/inputs/input.txt", "   ", true);
    }

    if second {
        if first {
            println!();
        }
        println!("Part 2:");
        part2::execute("%%PACKAGE_NAME%%/inputs/example2.txt", "    Example", true);
        part2::execute("%%PACKAGE_NAME%%/inputs/input.txt", "   ", true);
    }
}
