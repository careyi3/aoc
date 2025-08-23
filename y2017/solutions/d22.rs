use std::collections::{HashMap, HashSet};

use utils::{file_reader, harness::Solve};

pub struct D22;

impl Solve for D22 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let mut map: HashSet<(i32, i32)> = HashSet::new();
        let mut x_max = 0;
        let mut y_max = 0;
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    map.insert((x as i32, y as i32));
                }
                x_max = x as i32;
            }
            y_max = y as i32;
        }
        let mut x = x_max / 2;
        let mut y = y_max / 2;
        let mut dir = 1;
        let mut counter = 0;
        for _ in 0..10000 {
            (x, y, dir) = step(&mut map, &mut counter, x, y, dir);
        }

        return counter.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let mut x_max = 0;
        let mut y_max = 0;
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    map.insert((x as i32, y as i32), 'i');
                }
                x_max = x as i32;
            }
            y_max = y as i32;
        }
        let mut x = x_max / 2;
        let mut y = y_max / 2;
        let mut dir = 1;
        let mut counter = 0;
        for _ in 0..10000000 {
            (x, y, dir) = step_evolved(&mut map, &mut counter, x, y, dir);
        }

        return counter.to_string();
    }
}

fn step_evolved(
    map: &mut HashMap<(i32, i32), char>,
    counter: &mut i32,
    x: i32,
    y: i32,
    dir: i32,
) -> (i32, i32, i32) {
    let dirs = [[-1, 0], [0, -1], [1, 0], [0, 1]];
    let new_dir: i32;
    if map.contains_key(&(x, y)) {
        match map[&(x, y)] {
            'w' => {
                let entry = map.get_mut(&(x, y)).unwrap();
                *entry = 'i';
                *counter += 1;
                new_dir = dir;
            }
            'i' => {
                let entry = map.get_mut(&(x, y)).unwrap();
                *entry = 'f';
                new_dir = ((dir + 1) % 4 + 4) % 4;
            }
            'f' => {
                map.remove(&(x, y));
                new_dir = ((dir + 2) % 4 + 4) % 4;
            }
            _ => {
                new_dir = dir;
            }
        }
    } else {
        map.insert((x, y), 'w');
        new_dir = ((dir - 1) % 4 + 4) % 4;
    }
    let new_x = dirs[new_dir as usize][0] + x;
    let new_y: i32 = dirs[new_dir as usize][1] + y;
    return (new_x, new_y, new_dir);
}

fn step(
    map: &mut HashSet<(i32, i32)>,
    counter: &mut i32,
    x: i32,
    y: i32,
    dir: i32,
) -> (i32, i32, i32) {
    let dirs = [[-1, 0], [0, -1], [1, 0], [0, 1]];
    let new_dir: i32;
    if map.contains(&(x, y)) {
        map.remove(&(x, y));
        new_dir = ((dir + 1) % 4 + 4) % 4;
    } else {
        map.insert((x, y));
        *counter += 1;
        new_dir = ((dir - 1) % 4 + 4) % 4;
    }
    let new_x = dirs[new_dir as usize][0] + x;
    let new_y: i32 = dirs[new_dir as usize][1] + y;
    return (new_x, new_y, new_dir);
}
