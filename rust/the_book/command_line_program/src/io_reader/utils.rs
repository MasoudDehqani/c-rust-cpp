use std::fs;
use std::io::Error;

pub fn reader(file_path: &str) -> Result<String, Error> {
    fs::read_to_string(file_path)
}
