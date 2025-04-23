use utils::{file_reader, harness::Solve};

use md5::compute;

pub struct D05;

impl Solve for D05 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let secret = input.first().unwrap();

        let mut digits: Vec<char> = vec![];
        let mut digest;
        let mut i = 0;
        loop {
            let message = format!("{}{}", secret, i);
            digest = compute(message);
            let hex_digest = format!("{:x}", digest);
            if hex_digest.starts_with("00000") {
                digits.push(hex_digest.chars().nth(5).unwrap());
            }
            if digits.len() == 8 {
                break;
            }
            i += 1;
        }

        return digits.iter().collect();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let secret = input.first().unwrap();

        let mut digits = ['x', 'x', 'x', 'x', 'x', 'x', 'x', 'x'];
        let mut digest;
        let mut i = 0;
        loop {
            let message = format!("{}{}", secret, i);
            digest = compute(message);
            let hex_digest = format!("{:x}", digest);
            if hex_digest.starts_with("00000") {
                let val = hex_digest.chars().nth(6).unwrap();
                let position = hex_digest.chars().nth(5).unwrap();
                if position.is_alphabetic() {
                    i += 1;
                    continue;
                }
                let idx: usize = position.to_digit(10).unwrap() as usize;
                if idx > 7 {
                    i += 1;
                    continue;
                }
                if digits[idx] == 'x' {
                    digits[idx] = val;
                } else {
                    i += 1;
                    continue;
                }
            }
            if !digits.contains(&'x') {
                break;
            }
            i += 1;
        }

        return digits.iter().collect();
    }
}
