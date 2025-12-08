use std::fs::{read_to_string, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_line(filename: impl AsRef<Path>) -> String {
    read_to_string(&filename)
        .expect("Failed to read file")
        .lines()
        .next()
        .expect("File is empty")
        .to_string()
}

pub fn try_read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    Ok(read_to_string(filename)?
        .lines()
        .map(String::from)
        .collect())
}

pub fn try_read_line(filename: impl AsRef<Path>) -> io::Result<String> {
    read_to_string(filename)?
        .lines()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "File is empty"))
        .map(String::from)
}

pub fn lines_iter(
    filename: impl AsRef<Path>,
) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
