use anyhow::Result;
use utils::{file_reader, harness::SolveResult};

pub struct D17;

impl SolveResult for D17 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        return Ok(input.first().unwrap().to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        return Ok(input.first().unwrap().to_string());
    }
}
