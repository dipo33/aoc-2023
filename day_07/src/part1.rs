use std::path::Path;

use crate::common;

pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    common::solve(path, name, print, false)
}
