use std::fs;
use std::path::Path;

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");
}
