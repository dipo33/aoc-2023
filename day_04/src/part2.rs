use std::fs;
use std::path::Path;

#[allow(unused_variables)]
pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let result: u32 = u32::default();

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
