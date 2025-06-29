use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D05;

impl Solve for D05 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut idx = 0;
        let mut instructions: HashMap<i32, i32> = HashMap::new();
        for input in inputs {
            let value: i32 = input.parse().unwrap();
            instructions.insert(idx, value);
            idx += 1;
        }

        let mut ins = 0;
        let mut steps = 0;
        loop {
            if !instructions.contains_key(&ins) {
                break;
            }
            let offset = instructions[&ins];
            instructions.insert(ins, offset + 1);
            ins += offset;
            steps += 1;
        }

        return steps.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut idx = 0;
        let mut instructions: HashMap<i32, i32> = HashMap::new();
        for input in inputs {
            let value: i32 = input.parse().unwrap();
            instructions.insert(idx, value);
            idx += 1;
        }

        let mut ins = 0;
        let mut steps = 0;
        loop {
            if !instructions.contains_key(&ins) {
                break;
            }
            let offset = instructions[&ins];
            if offset >= 3 {
                instructions.insert(ins, offset - 1);
            } else {
                instructions.insert(ins, offset + 1);
            }
            ins += offset;
            steps += 1;
        }

        return steps.to_string();
    }
}
