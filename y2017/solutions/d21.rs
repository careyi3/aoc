use utils::{file_reader, harness::Solve};

pub struct D21;

impl Solve for D21 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        return input.first().unwrap().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        return input.first().unwrap().to_string();
    }
}

