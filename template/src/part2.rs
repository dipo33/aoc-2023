use std::fs;
use std::path::Path;

#[allow(unused_variables)]
pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> %%RESULT_TYPE%% {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let result: %%RESULT_TYPE%% = %%RESULT_TYPE%%::default();

    if print {
        println!("{} Result: {}", name, result);
    }

    result
}
