use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use utils::{file_reader, harness::Solve};

pub struct D06;

impl Solve for D06 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let initial_blocks: Vec<i32> = input
            .first()
            .unwrap()
            .split("	")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut memory: HashMap<i32, i32> = HashMap::new();
        let mut max = 0;
        let mut max_idx = 0;
        let mut len = 0;
        for (idx, block) in initial_blocks.iter().enumerate() {
            if *block > max {
                max = *block;
                max_idx = idx as i32
            }
            memory.insert(idx as i32, *block);
            len = idx as i32;
        }
        len += 1;

        let mut cycles = 0;
        let mut history: HashSet<String> = HashSet::new();
        loop {
            let start_idx = max_idx + 1;
            memory.insert(max_idx, 0);
            for i in 0..max {
                let idx = (start_idx + i) % len;
                memory.entry(idx).and_modify(|f| *f += 1);
            }

            let hash = hash_memory(&memory);
            if history.contains(&hash) {
                cycles += 1;
                break;
            } else {
                history.insert(hash);
            }

            max = 0;
            max_idx = 0;
            for idx in 0..len {
                let val = memory[&idx];
                if val > max {
                    max = val;
                    max_idx = idx;
                }
            }

            cycles += 1;
        }

        return cycles.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let initial_blocks: Vec<i32> = input
            .first()
            .unwrap()
            .split("	")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut memory: HashMap<i32, i32> = HashMap::new();
        let mut max = 0;
        let mut max_idx = 0;
        let mut len = 0;
        for (idx, block) in initial_blocks.iter().enumerate() {
            if *block > max {
                max = *block;
                max_idx = idx as i32
            }
            memory.insert(idx as i32, *block);
            len = idx as i32;
        }
        len += 1;

        let mut cycles = 0;
        let ans;
        let mut history: HashMap<String, i32> = HashMap::new();
        loop {
            let start_idx = max_idx + 1;
            memory.insert(max_idx, 0);
            for i in 0..max {
                let idx = (start_idx + i) % len;
                memory.entry(idx).and_modify(|f| *f += 1);
            }

            let hash = hash_memory(&memory);
            if history.contains_key(&hash) {
                let last = history[&hash];
                ans = cycles - last;
                break;
            } else {
                history.insert(hash, cycles);
            }

            max = 0;
            max_idx = 0;
            for idx in 0..len {
                let val = memory[&idx];
                if val > max {
                    max = val;
                    max_idx = idx;
                }
            }

            cycles += 1;
        }

        return ans.to_string();
    }
}

fn hash_memory(memory: &HashMap<i32, i32>) -> String {
    let len = memory.len() as i32;
    let mut vals: Vec<i32> = vec![];
    for idx in 0..len {
        vals.push(memory[&idx]);
    }
    return vals.iter().join(":");
}
