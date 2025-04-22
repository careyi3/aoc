use unescaper::unescape;
use utils::{file_reader, harness::Solve};

pub struct D08;

impl Solve for D08 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut total_unescaped = 0;
        let mut total_escaped = 0;
        for input in inputs {
            total_unescaped += input.chars().count();
            let unescaped = unescape(&input).unwrap();
            total_escaped += unescaped.chars().count() - 2;
        }

        return (total_unescaped - total_escaped).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut total_reescaped = 0;
        let mut total_escaped = 0;
        for input in inputs {
            total_escaped += input.chars().count();
            let reescaped = format!("{:?}", input);
            total_reescaped += reescaped.chars().count();
        }

        return (total_reescaped - total_escaped).to_string();
    }
}
