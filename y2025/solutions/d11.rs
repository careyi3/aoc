use std::collections::HashMap;

use anyhow::Result;
use utils::{file_reader, harness::SolveResult};

pub struct D11;

impl SolveResult for D11 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for line in input {
            let key_values: Vec<&str> = line.split(": ").collect();
            let key = key_values[0].to_string();
            let values: Vec<String> = key_values[1].split(" ").map(|x| x.to_string()).collect();
            map.insert(key, values);
        }

        let mut cache: HashMap<String, i64> = HashMap::new();
        let count = walk(&map, &mut cache, "you".to_string(), "out".to_string());

        return Ok(count.to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for line in input {
            let key_values: Vec<&str> = line.split(": ").collect();
            let key = key_values[0].to_string();
            let values: Vec<String> = key_values[1].split(" ").map(|x| x.to_string()).collect();
            map.insert(key, values);
        }

        let ans: i64 = [["svr", "fft"], ["fft", "dac"], ["dac", "out"]]
            .iter()
            .fold(1, |mut acc, x| {
                let mut cache: HashMap<String, i64> = HashMap::new();
                let val = walk(&map, &mut cache, x[0].to_string(), x[1].to_string());
                acc *= val;
                acc
            });

        return Ok(ans.to_string());
    }
}

fn walk(
    map: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, i64>,
    next: String,
    dest: String,
) -> i64 {
    if cache.contains_key(&next) {
        return cache[&next];
    }
    if next == dest {
        return 1;
    }
    if !map.contains_key(&next) {
        return 0;
    }
    let mut sum = 0;
    for key in &map[&next] {
        sum += walk(map, cache, key.to_string(), dest.to_string());
    }
    cache.insert(next, sum);
    return sum;
}
