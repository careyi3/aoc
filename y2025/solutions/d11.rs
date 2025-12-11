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

        let mut count = 0;
        walk(&map, "you".to_string(), &mut count);

        return Ok(count.to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        return Ok(input.first().unwrap().to_string());
    }
}

fn walk(map: &HashMap<String, Vec<String>>, next: String, count: &mut i32) {
    if next == "out".to_string() {
        *count += 1;
        return;
    }
    if !map.contains_key(&next) {
        return;
    }
    for key in &map[&next] {
        walk(map, key.to_string(), count);
    }
}
