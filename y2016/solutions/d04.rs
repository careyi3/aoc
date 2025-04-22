use utils::{file_reader, harness::Solve};

pub struct D04;

impl Solve for D04 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        return input.first().unwrap().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        return input.first().unwrap().to_string();
    }
}
