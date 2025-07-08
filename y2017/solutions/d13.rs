use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D13;

impl Solve for D13 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut depths: HashMap<i32, i32> = HashMap::new();
        for input in inputs {
            let tokens: Vec<i32> = input
                .split(": ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            depths.insert(tokens[0], tokens[1]);
        }

        let max = depths.keys().max().unwrap() + 1;
        let mut ans = 0;
        for i in 0..max {
            if depths.contains_key(&i) {
                let range = depths[&i];
                let cycle_len = 2 * (range - 1);
                if i % cycle_len == 0 {
                    ans += range * i;
                }
            } else {
                continue;
            }
        }

        return ans.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut depths: HashMap<i32, i32> = HashMap::new();
        for input in inputs {
            let tokens: Vec<i32> = input
                .split(": ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            depths.insert(tokens[0], tokens[1]);
        }

        let max = depths.keys().max().unwrap() + 1;
        let mut start = 1;
        loop {
            let mut ans = 0;
            'inner: for i in 0..max {
                if depths.contains_key(&i) {
                    let range = depths[&i];
                    let cycle_len = 2 * (range - 1);
                    if (i + start) % cycle_len == 0 {
                        ans += range * (i + 1);
                        break 'inner;
                    }
                } else {
                    continue 'inner;
                }
            }
            if ans == 0 {
                break;
            }
            start += 1;
        }

        return start.to_string();
    }
}
