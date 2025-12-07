use anyhow::Result;
use std::collections::{HashMap, HashSet};
use utils::{
    annimations::save_annimation_data, annimations::Frame, file_reader, harness::SolveResult,
};

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
        let mut visited: Vec<(i32, i32)> = vec![];
        let mut frames: Vec<Frame> = vec![];
        walk(
            &map,
            start.0,
            start.1,
            &mut splits,
            &mut visited,
            &mut frames,
        );

        save_annimation_data("2025_day_7".to_string(), 16, 16, frames, 200.00);

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

fn walk(
    map: &HashMap<(i32, i32), char>,
    x: i32,
    y: i32,
    splits: &mut HashSet<(i32, i32)>,
    visited: &mut Vec<(i32, i32)>,
    frames: &mut Vec<Frame>,
) {
    if !map.contains_key(&(x, y)) || splits.contains(&(x, y)) || visited.contains(&(x, y)) {
        return;
    }
    visited.push((x, y));
    frames.push(generate_frame(map, visited));
    if map[&(x, y)] != '^' {
        walk(map, x, y + 1, splits, visited, frames);
    } else {
        splits.insert((x, y));
        walk(map, x - 1, y + 1, splits, visited, frames);
        walk(map, x + 1, y + 1, splits, visited, frames);
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

fn generate_frame(map: &HashMap<(i32, i32), char>, visited: &Vec<(i32, i32)>) -> Frame {
    let mut grid: Vec<Vec<u8>> = vec![];
    for y in 0..16 {
        let mut line = vec![];
        for x in 0..16 {
            if map.contains_key(&(x, y)) {
                match *map.get(&(x, y)).unwrap() {
                    '.' => {
                        line.push(0);
                    }
                    '^' => {
                        line.push(2);
                    }
                    _ => {}
                }
            } else {
                line.push(0);
            }
        }
        grid.push(line);
    }

    let highlighted = visited
        .iter()
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect();

    return Frame {
        step: visited.len(),
        message: format!("Iteration: {}", visited.len()),
        grid,
        highlighted,
    };
}
