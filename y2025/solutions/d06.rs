use anyhow::Result;
use std::collections::HashMap;
use utils::{file_reader, harness::SolveResult};

pub struct D06;

impl SolveResult for D06 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut problems: Vec<Vec<i64>> = vec![];
        let mut operators: Vec<String> = vec![];
        for line in input {
            for (idx, num_str) in line.split_whitespace().enumerate() {
                if problems.len() < idx + 1 {
                    problems.push(vec![]);
                }
                match num_str.parse::<i64>() {
                    Ok(num) => {
                        problems[idx].push(num);
                    }
                    Err(_) => {
                        operators.push(num_str.to_string());
                    }
                }
            }
        }

        let mut sum = 0;
        for (idx, operator) in operators.iter().enumerate() {
            match operator.as_str() {
                "+" => {
                    sum += problems[idx]
                        .iter()
                        .copied()
                        .reduce(|acc, x| acc + x)
                        .unwrap_or(0);
                }
                "*" => {
                    sum += problems[idx]
                        .iter()
                        .copied()
                        .reduce(|acc, x| acc * x)
                        .unwrap_or(0);
                }
                _ => {}
            }
        }

        return Ok(sum.to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut problems: Vec<Vec<char>> = vec![];
        let mut operators: HashMap<usize, char> = HashMap::new();
        let mut indexes: Vec<usize> = vec![];
        let mut max_width: usize = 0;
        for line in input {
            let mut chars: Vec<char> = vec![];
            for (idx, c) in line.chars().enumerate() {
                match c {
                    '+' => {
                        operators.insert(idx, c);
                        indexes.push(idx);
                    }
                    '*' => {
                        operators.insert(idx, c);
                        indexes.push(idx);
                    }
                    _ => {
                        chars.push(c);
                    }
                }
            }
            if chars.len() > max_width {
                max_width = chars.len();
            }
            problems.push(chars);
        }
        indexes.push(max_width + 1);

        let mut sum = 0;

        for window in indexes.windows(2) {
            let mut nums = vec![];
            for x in window[0]..window[1] - 1 {
                let mut num_string = "".to_string();
                for y in 0..problems.len() {
                    if x < problems[y].len() {
                        num_string.push_str(&problems[y][x].to_string());
                    }
                }
                let num = num_string.trim().parse::<i64>()?;
                nums.push(num);
            }
            let op = operators[&window[0]];
            match op {
                '+' => {
                    sum += nums.iter().copied().reduce(|acc, x| acc + x).unwrap_or(0);
                }
                '*' => {
                    sum += nums.iter().copied().reduce(|acc, x| acc * x).unwrap_or(0);
                }
                _ => {}
            }
        }

        return Ok(sum.to_string());
    }
}
