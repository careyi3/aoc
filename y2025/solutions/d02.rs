use anyhow::Result;
use std::collections::HashMap;
use utils::{file_reader, harness::SolveResult};

pub struct D02;

impl SolveResult for D02 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let ranges: Vec<Vec<usize>> = input
            .first()
            .unwrap()
            .split(',')
            .map(|x| x.split('-').map(|y| y.parse::<usize>().unwrap()).collect())
            .collect();

        let mut sum = 0;

        for range in ranges {
            for num in range[0]..=range[1] {
                let stringy_num = num.to_string();
                let mid = stringy_num.len() / 2;
                let (left, right) = stringy_num.split_at(mid);
                if left == right {
                    sum += num;
                }
            }
        }

        return Ok(sum.to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let ranges: Vec<Vec<usize>> = input
            .first()
            .unwrap()
            .split(',')
            .map(|x| x.split('-').map(|y| y.parse::<usize>().unwrap()).collect())
            .collect();

        let mut sum = 0;

        for range in ranges {
            for num in range[0]..=range[1] {
                let stringy_num = num.to_string();
                let mid = stringy_num.len() / 2;
                let mut patterns: Vec<String> = vec![];
                for i in 0..mid {
                    patterns.push(stringy_num[0..=i].to_string());
                }
                let mut counts: HashMap<String, usize> = HashMap::new();
                for pattern in patterns {
                    let count = stringy_num
                        .as_bytes()
                        .chunks(pattern.len())
                        .filter(|&s| s == pattern.as_bytes())
                        .count();
                    counts.insert(pattern.clone(), count);
                }
                'inner: for (pattern, count) in counts {
                    if pattern.len() * count == stringy_num.len() {
                        sum += num;
                        break 'inner;
                    }
                }
            }
        }

        return Ok(sum.to_string());
    }
}
