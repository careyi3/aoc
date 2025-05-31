use fancy_regex::Regex;
use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D15;

impl Solve for D15 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let num_positions_reg = Regex::new(r"(\d+) positions;").unwrap();
        let start_position_reg = Regex::new(r"position (\d+)\.").unwrap();
        let mut discs: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut disc_num = 1;
        for input in inputs {
            let num_positions = num_positions_reg
                .captures(&input)
                .unwrap()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let start_position = start_position_reg
                .captures(&input)
                .unwrap()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            discs.insert(disc_num, (num_positions, start_position));
            disc_num += 1;
        }

        let mut t = 0;
        let ans;
        'outer: loop {
            let mut correct: Vec<bool> = vec![];
            for (id, (num_positions, start_position)) in &discs {
                let position_at_time = (start_position + t + id) % num_positions;
                let c = position_at_time == 0;
                correct.push(c);
            }
            if correct.iter().all(|x| *x) {
                ans = t;
                break 'outer;
            }
            t += 1;
        }

        return ans.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let num_positions_reg = Regex::new(r"(\d+) positions;").unwrap();
        let start_position_reg = Regex::new(r"position (\d+)\.").unwrap();
        let mut discs: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut disc_num = 1;
        for input in inputs {
            let num_positions = num_positions_reg
                .captures(&input)
                .unwrap()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let start_position = start_position_reg
                .captures(&input)
                .unwrap()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            discs.insert(disc_num, (num_positions, start_position));
            disc_num += 1;
        }

        discs.insert(7, (11, 0));

        let mut t = 0;
        let ans;
        'outer: loop {
            let mut correct: Vec<bool> = vec![];
            for (id, (num_positions, start_position)) in &discs {
                let position_at_time = (start_position + t + id) % num_positions;
                let c = position_at_time == 0;
                correct.push(c);
            }
            if correct.iter().all(|x| *x) {
                ans = t;
                break 'outer;
            }
            t += 1;
        }

        return ans.to_string();
    }
}
