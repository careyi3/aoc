use anyhow::Result;
use itertools::Itertools;
use utils::{file_reader, harness::SolveResult};

pub struct D03;

impl SolveResult for D03 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        return solve(input, 2);
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        return solve(input, 12);
    }
}

fn solve(input: Vec<String>, num_batteries: usize) -> Result<String> {
    let mut sum = 0;
    for line in input {
        let num_nums = line.len();
        let mut nums: Vec<u32> = vec![];
        for (idx, num) in line.chars().map(|x| x.to_digit(10).unwrap()).enumerate() {
            loop {
                if nums.len() == 0 {
                    nums.push(num);
                    break;
                }
                let nums_left = num_nums - idx;
                let nums_to_fill = num_batteries - nums.len();
                if *nums.last().unwrap() < num && nums_left > nums_to_fill {
                    nums.pop();
                    continue;
                }
                if nums.len() == num_batteries {
                    break;
                } else {
                    nums.push(num);
                    break;
                }
            }
        }
        let n = nums.iter().map(|x| x.to_string()).join("");
        sum += n.parse::<i64>()?;
    }
    return Ok(sum.to_string());
}
