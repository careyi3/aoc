use md5::compute;

use utils::{file_reader, harness::Solve};

pub struct D04;

impl Solve for D04 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let secret = input.first().unwrap();

        let mut digest;
        for i in 1..10000000 {
            let message = format!("{}{}", secret, i);
            digest = compute(message);
            let hex_digest = format!("{:x}", digest);
            if hex_digest.starts_with("00000") {
                return i.to_string();
            }
        }

        return "".to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let secret = input.first().unwrap();

        let mut digest;
        for i in 1..10000000 {
            let message = format!("{}{}", secret, i);
            digest = compute(message);
            let hex_digest = format!("{:x}", digest);
            if hex_digest.starts_with("000000") {
                return i.to_string();
            }
        }

        return "".to_string();
    }
}
