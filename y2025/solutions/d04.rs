use std::collections::HashSet;

use anyhow::Result;
use utils::{
    annimations::save_annimation_data, annimations::Frame, file_reader, harness::SolveResult,
};

pub struct D04;

const DIRS: [[i32; 2]; 8] = [
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1],
    [-1, 1],
    [1, -1],
    [-1, -1],
    [1, 1],
];

impl SolveResult for D04 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut map: HashSet<(i32, i32)> = HashSet::new();
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    map.insert((x as i32, y as i32));
                }
            }
        }

        let mut count = 0;
        for (x, y) in &map {
            if count_adjacent(&map, *x, *y) < 4 {
                count += 1;
            }
        }

        return Ok(count.to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut map: HashSet<(i32, i32)> = HashSet::new();
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    map.insert((x as i32, y as i32));
                }
            }
        }

        let mut count = 0;
        let mut frames: Vec<Frame> = vec![];
        let mut step = 0;
        loop {
            let mut to_remove: Vec<(i32, i32)> = vec![];
            for (x, y) in &map {
                if count_adjacent(&map, *x, *y) < 4 {
                    count += 1;
                    to_remove.push((*x, *y));
                }
            }
            if to_remove.len() == 0 {
                frames.push(generate_frame(&map, &to_remove, step));
                break;
            }
            for coords in &to_remove {
                map.remove(&coords);
            }
            frames.push(generate_frame(&map, &to_remove, step));
            step += 1;
        }

        save_annimation_data("2025_day_4".to_string(), 140, 140, frames, 250.0);

        return Ok(count.to_string());
    }
}

fn count_adjacent(map: &HashSet<(i32, i32)>, x: i32, y: i32) -> i32 {
    let mut count = 0;
    for dir in DIRS {
        let new_x = x + dir[0];
        let new_y = y + dir[1];
        if map.contains(&(new_x, new_y)) {
            count += 1;
        }
    }
    return count;
}

fn generate_frame(map: &HashSet<(i32, i32)>, to_remove: &Vec<(i32, i32)>, step: usize) -> Frame {
    let mut grid: Vec<Vec<u8>> = vec![];
    for y in 0..140 {
        let mut line = vec![];
        for x in 0..140 {
            if map.contains(&(x, y)) {
                line.push(1);
            } else {
                line.push(0);
            }
        }
        grid.push(line);
    }

    let highlighted = to_remove
        .iter()
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect();

    return Frame {
        step,
        message: format!("Iteration: {}", step),
        grid,
        highlighted,
    };
}
