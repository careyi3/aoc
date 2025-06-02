use utils::{file_reader, harness::Solve};

pub struct D16;

impl Solve for D16 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let input = inputs.first().unwrap();

        let mut a = input.clone().to_string();
        while a.len() < 272 {
            let test = a.clone();
            a.push('0');
            for seq in test.chars().rev() {
                if seq == '1' {
                    a.push('0');
                } else {
                    a.push('1');
                }
            }
        }

        let mut checksum: String = a[..272].to_string();
        while checksum.len() % 2 == 0 {
            let mut temp: String = "".to_string();
            let checksum_chars: Vec<char> = checksum.chars().collect();
            for i in (0..checksum.len() - 1).step_by(2) {
                if checksum_chars[i] == checksum_chars[i + 1] {
                    temp.push('1');
                } else {
                    temp.push('0');
                }
            }
            checksum = temp;
        }

        return checksum;
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let input = inputs.first().unwrap();

        let mut a = input.clone().to_string();
        while a.len() < 35651584 {
            let test = a.clone();
            a.push('0');
            for seq in test.chars().rev() {
                if seq == '1' {
                    a.push('0');
                } else {
                    a.push('1');
                }
            }
        }

        let mut checksum: String = a[..35651584].to_string();
        while checksum.len() % 2 == 0 {
            let mut temp: String = "".to_string();
            let checksum_chars: Vec<char> = checksum.chars().collect();
            for i in (0..checksum.len() - 1).step_by(2) {
                if checksum_chars[i] == checksum_chars[i + 1] {
                    temp.push('1');
                } else {
                    temp.push('0');
                }
            }
            checksum = temp;
        }

        return checksum;
    }
}
