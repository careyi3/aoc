use std::fs::read_to_string;
use std::path::Path;

pub fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_line(filename: impl AsRef<Path>) -> String {
    read_lines(&filename)
        .into_iter()
        .next()
        .expect("File is empty")
}

pub fn try_read_lines(filename: impl AsRef<Path>) -> std::io::Result<Vec<String>> {
    Ok(read_to_string(filename)?
        .lines()
        .map(String::from)
        .collect())
}

pub fn try_read_line(filename: impl AsRef<Path>) -> std::io::Result<String> {
    try_read_lines(filename)?
        .into_iter()
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "File is empty"))
}
