use fancy_regex::Regex;
use utils::{file_reader, harness::Solve};

pub struct D11;

impl Solve for D11 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let greg = Regex::new(r"(\w+) generator").unwrap();
        let _creg = Regex::new(r"(\w+)-compatible microchip").unwrap();

        for input in inputs {
            for cap in greg.captures_iter(&input) {
                let captures = cap.unwrap();
                for capture in captures.iter() {
                    let _mat = capture.unwrap();
                }
            }
        }

        return "".to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        return input.first().unwrap().to_string();
    }
}
