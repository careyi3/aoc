use std::collections::HashMap;

use itertools::Itertools;
use utils::{file_reader, harness::Solve};

pub struct D17;

impl Solve for D17 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        let nums: Vec<i32> = input
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut count = 0;
        for i in 1..=nums.len() {
            for perm in nums.iter().combinations(i) {
                let values: Vec<i32> = perm.into_iter().map(|a| *a).collect();
                if values.into_iter().sum::<i32>() == 150 {
                    count += 1
                }
            }
        }

        return count.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        let nums: Vec<i32> = input
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut results: HashMap<usize, i32> = HashMap::new();
        for i in 1..=nums.len() {
            for perm in nums.iter().combinations(i) {
                let values: Vec<i32> = perm.into_iter().map(|a| *a).collect();
                if values.into_iter().sum::<i32>() == 150 {
                    results.entry(i).and_modify(|f| *f += 1).or_insert(1);
                }
            }
        }

        let min = results.keys().min().unwrap();

        return results[min].to_string();
    }
}
