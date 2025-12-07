use anyhow::Result;
use std::collections::{HashMap, HashSet};
use utils::{file_reader, harness::SolveResult};

pub struct D07;

impl SolveResult for D07 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let mut start = (0, 0);
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = (x as i32, y as i32);
                }
                map.insert((x as i32, y as i32), c);
            }
        }

        let mut splits: HashSet<(i32, i32)> = HashSet::new();
        walk(&map, start.0, start.1, &mut splits);

        return Ok(splits.len().to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let mut start = (0, 0);
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = (x as i32, y as i32);
                }
                map.insert((x as i32, y as i32), c);
            }
        }

        let mut cache: HashMap<(i32, i32), i64> = HashMap::new();
        let count = walk_quantum(&map, start.0, start.1, &mut cache);

        return Ok(count.to_string());
    }
}

fn walk(map: &HashMap<(i32, i32), char>, x: i32, y: i32, splits: &mut HashSet<(i32, i32)>) {
    if !map.contains_key(&(x, y)) || splits.contains(&(x, y)) {
        return;
    }
    if map[&(x, y)] != '^' {
        walk(map, x, y + 1, splits);
    } else {
        splits.insert((x, y));
        walk(map, x - 1, y + 1, splits);
        walk(map, x + 1, y + 1, splits);
    }
}

fn walk_quantum(
    map: &HashMap<(i32, i32), char>,
    x: i32,
    y: i32,
    cache: &mut HashMap<(i32, i32), i64>,
) -> i64 {
    if cache.contains_key(&(x, y)) {
        return cache[&(x, y)];
    }
    if !map.contains_key(&(x, y)) {
        return 1;
    }
    let mut count = 0;
    if map[&(x, y)] != '^' {
        count = walk_quantum(map, x, y + 1, cache);
    } else {
        count += walk_quantum(map, x - 1, y + 1, cache);
        count += walk_quantum(map, x + 1, y + 1, cache);
    }
    cache.insert((x, y), count);
    return count;
}
