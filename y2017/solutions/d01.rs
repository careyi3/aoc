use utils::{file_reader, harness::Solve};

pub struct D01;

impl Solve for D01 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let chars: Vec<char> = inputs.first().unwrap().chars().collect();

        let mut sum = 0;
        for i in 0..chars.len() {
            let idx1 = i;
            let idx2 = (i + 1) % chars.len();

            if chars[idx1] == chars[idx2] {
                sum += chars[idx1].to_digit(10).unwrap();
            }
        }

        return sum.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let chars: Vec<char> = inputs.first().unwrap().chars().collect();

        let mut sum = 0;
        for i in 0..chars.len() {
            let idx1 = i;
            let idx2 = (i + chars.len() / 2) % chars.len();

            if chars[idx1] == chars[idx2] {
                sum += chars[idx1].to_digit(10).unwrap();
            }
        }

        return sum.to_string();
    }
}
