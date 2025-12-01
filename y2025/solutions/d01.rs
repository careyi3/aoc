use anyhow::Result;
use utils::{file_reader, harness::SolveResult};

pub struct D01;

impl SolveResult for D01 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut num = 50;
        let mut count = 0;
        for line in input {
            let chars: Vec<char> = line.chars().collect();
            let mut dir = -1;
            if chars[0] == 'R' {
                dir = 1;
            }
            let number = &chars[1..];
            let offset: i32 = number.iter().collect::<String>().parse()?;

            num = (num + (offset * dir)).rem_euclid(100);

            if num == 0 {
                count += 1;
            }
        }

        return Ok(count.to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut count = 0;
        let mut dial: [u32; 100] = std::array::from_fn(|i| i as u32);
        for line in input {
            let chars: Vec<char> = line.chars().collect();
            let number = &chars[1..];
            let offset: i32 = number.iter().collect::<String>().parse()?;

            if chars[0] == 'R' {
                for _ in 0..offset {
                    dial.rotate_right(1);
                    if dial[50] == 0 {
                        count += 1;
                    }
                }
            } else {
                for _ in 0..offset {
                    dial.rotate_left(1);
                    if dial[50] == 0 {
                        count += 1;
                    }
                }
            }
        }

        return Ok(count.to_string());
    }
}
