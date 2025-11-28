use std::collections::HashMap;

use anyhow::Result;
use utils::{file_reader, harness::SolveResult};

pub struct D16;

impl SolveResult for D16 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut ops: HashMap<String, Box<dyn FnMut(&mut [i32; 4], i32, i32, i32)>> = HashMap::new();
        setup_ops(&mut ops);

        let mut samples: HashMap<i32, i32> = HashMap::new();
        let mut before: [i32; 4] = [0, 0, 0, 0];
        let mut inputs: [i32; 4] = [0, 0, 0, 0];
        let mut after: [i32; 4] = [0, 0, 0, 0];
        for (idx, line) in input.iter().enumerate() {
            let id = (idx / 4) as i32;
            if idx % 4 == 0 {
                let nums: [i32; 4] = line
                    .split('[')
                    .nth(1)
                    .unwrap()
                    .split(']')
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                before = nums;
            }
            if idx % 4 == 1 {
                let nums: [i32; 4] = line
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                inputs = nums;
            }
            if idx % 4 == 2 {
                let nums: [i32; 4] = line
                    .split('[')
                    .nth(1)
                    .unwrap()
                    .split(']')
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                after = nums;
            }
            if idx % 4 == 3 {
                let mut count = 0;
                for op in ops.values_mut() {
                    let mut temp_before = before.clone();
                    op(&mut temp_before, inputs[1], inputs[2], inputs[3]);
                    if temp_before == after {
                        count += 1;
                    }
                }
                samples.insert(id, count);
            }
        }

        let mut ans = 0;
        for sample in samples.values() {
            if *sample >= 3 {
                ans += 1;
            }
        }

        return Ok(ans.to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut ops: HashMap<String, Box<dyn FnMut(&mut [i32; 4], i32, i32, i32)>> = HashMap::new();
        setup_ops(&mut ops);

        let mut op_map: HashMap<i32, String> = HashMap::new();
        let mut before: [i32; 4] = [0, 0, 0, 0];
        let mut inputs: [i32; 4] = [0, 0, 0, 0];
        let mut after: [i32; 4] = [0, 0, 0, 0];
        for (idx, line) in input.iter().enumerate() {
            let _id = (idx / 4) as i32;
            if idx % 4 == 0 {
                let nums: [i32; 4] = line
                    .split('[')
                    .nth(1)
                    .unwrap()
                    .split(']')
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                before = nums;
            }
            if idx % 4 == 1 {
                let nums: [i32; 4] = line
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                inputs = nums;
            }
            if idx % 4 == 2 {
                let nums: [i32; 4] = line
                    .split('[')
                    .nth(1)
                    .unwrap()
                    .split(']')
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                after = nums;
            }
            if idx % 4 == 3 {
                let mut matches: Vec<(String, i32)> = vec![];
                for (key, op) in ops.iter_mut() {
                    let mut temp_before = before.clone();
                    op(&mut temp_before, inputs[1], inputs[2], inputs[3]);
                    if temp_before == after {
                        if !op_map.contains_key(&inputs[0]) {
                            matches.push((key.to_string(), inputs[0]));
                        }
                    }
                }
                if matches.len() == 1 {
                    let (val, key) = &matches[0];
                    op_map.insert(*key, val.to_string());
                }
            }
        }

        let ans = 0;

        return Ok(ans.to_string());
    }
}

fn setup_ops(ops: &mut HashMap<String, Box<dyn FnMut(&mut [i32; 4], i32, i32, i32)>>) {
    ops.insert(
        "addr".into(),
        Box::new(|reg, a, b, c| reg[c as usize] = reg[a as usize] + reg[b as usize]),
    );
    ops.insert(
        "addi".into(),
        Box::new(|reg, a, b, c| reg[c as usize] = reg[a as usize] + b),
    );
    ops.insert(
        "mulr".into(),
        Box::new(|reg, a, b, c| reg[c as usize] = reg[a as usize] * reg[b as usize]),
    );
    ops.insert(
        "muli".into(),
        Box::new(|reg, a, b, c| reg[c as usize] = reg[a as usize] * b),
    );
    ops.insert(
        "banr".into(),
        Box::new(|reg, a, b, c| reg[c as usize] = reg[a as usize] & reg[b as usize]),
    );
    ops.insert(
        "bani".into(),
        Box::new(|reg, a, b, c| reg[c as usize] = reg[a as usize] & b),
    );
    ops.insert(
        "borr".into(),
        Box::new(|reg, a, b, c| reg[c as usize] = reg[a as usize] | reg[b as usize]),
    );
    ops.insert(
        "bori".into(),
        Box::new(|reg, a, b, c| reg[c as usize] = reg[a as usize] | b),
    );
    ops.insert(
        "setr".into(),
        Box::new(|reg, a, _b, c| reg[c as usize] = reg[a as usize]),
    );
    ops.insert("seti".into(), Box::new(|reg, a, _b, c| reg[c as usize] = a));
    ops.insert(
        "gtir".into(),
        Box::new(|reg, a, b, c| {
            if a > reg[b as usize] {
                reg[c as usize] = 1;
            } else {
                reg[c as usize] = 0;
            }
        }),
    );
    ops.insert(
        "gtri".into(),
        Box::new(|reg, a, b, c| {
            if reg[a as usize] > b {
                reg[c as usize] = 1;
            } else {
                reg[c as usize] = 0;
            }
        }),
    );
    ops.insert(
        "gtrr".into(),
        Box::new(|reg, a, b, c| {
            if reg[a as usize] > reg[b as usize] {
                reg[c as usize] = 1;
            } else {
                reg[c as usize] = 0;
            }
        }),
    );
    ops.insert(
        "eqir".into(),
        Box::new(|reg, a, b, c| {
            if a == reg[b as usize] {
                reg[c as usize] = 1;
            } else {
                reg[c as usize] = 0;
            }
        }),
    );
    ops.insert(
        "eqri".into(),
        Box::new(|reg, a, b, c| {
            if reg[a as usize] == b {
                reg[c as usize] = 1;
            } else {
                reg[c as usize] = 0;
            }
        }),
    );
    ops.insert(
        "eqrr".into(),
        Box::new(|reg, a, b, c| {
            if reg[a as usize] == reg[b as usize] {
                reg[c as usize] = 1;
            } else {
                reg[c as usize] = 0;
            }
        }),
    );
}
