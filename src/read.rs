use std::fs::read_to_string;
use std::path::Path;

pub fn read_all (path: &Path) -> String {
    let content = read_to_string(path).unwrap();
    content
}