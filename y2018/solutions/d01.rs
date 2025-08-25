use std::collections::HashSet;

use utils::{file_reader, harness::Solve};

pub struct D01;

impl Solve for D01 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let ans: i32 = inputs.iter().map(|x| x.parse::<i32>().unwrap()).sum();

        return ans.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut freqs: HashSet<i32> = HashSet::new();
        let mut freq = 0;
        'outer: loop {
            for f in inputs.iter().map(|x| x.parse::<i32>().unwrap()) {
                freq += f;
                if freqs.contains(&freq) {
                    break 'outer;
                }
                freqs.insert(freq);
            }
        }

        return freq.to_string();
    }
}
