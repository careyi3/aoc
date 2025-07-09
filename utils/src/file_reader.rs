use std::fs::read_to_string;

pub fn read_lines(filename: &String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_line(filename: &String) -> String {
    return read_lines(filename).first().unwrap().to_string();
}
