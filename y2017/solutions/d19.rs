use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D19;

impl Solve for D19 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let dir = 3;
        let mut start_x: i32 = 0;
        let start_y: i32 = 0;
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        for (y, line) in inputs.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != ' ' {
                    if y == 0 {
                        start_x = x as i32;
                    }
                    map.insert((x as i32, y as i32), c);
                }
            }
        }

        let mut path = "".to_string();
        walk(&map, dir, start_x, start_y, &mut path);

        return path;
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let dir = 3;
        let mut start_x: i32 = 0;
        let start_y: i32 = 0;
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        for (y, line) in inputs.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != ' ' {
                    if y == 0 {
                        start_x = x as i32;
                    }
                    map.insert((x as i32, y as i32), c);
                }
            }
        }

        let mut count = 0;
        walk_step_count(&map, dir, start_x, start_y, &mut count);

        return count.to_string();
    }
}

fn walk(map: &HashMap<(i32, i32), char>, dir: i32, x: i32, y: i32, path: &mut String) {
    if map[&(x, y)] != '+' && map[&(x, y)] != '|' && map[&(x, y)] != '-' {
        path.push(map[&(x, y)]);
    }
    let dirs = [[-1, 0], [0, -1], [1, 0], [0, 1]];
    let next_x = x + dirs[dir as usize][0];
    let next_y = y + dirs[dir as usize][1];
    if map.contains_key(&(next_x, next_y)) {
        walk(map, dir, next_x, next_y, path);
    } else {
        for (new_dir, xy) in dirs.iter().enumerate() {
            if new_dir == ((dir + 2) % 4) as usize || new_dir == dir as usize {
                continue;
            }
            let next_x = x + xy[0];
            let next_y = y + xy[1];
            if map.contains_key(&(next_x, next_y)) {
                walk(map, new_dir as i32, next_x, next_y, path);
            }
        }
    }
}

fn walk_step_count(map: &HashMap<(i32, i32), char>, dir: i32, x: i32, y: i32, count: &mut i32) {
    *count += 1;
    let dirs = [[-1, 0], [0, -1], [1, 0], [0, 1]];
    let next_x = x + dirs[dir as usize][0];
    let next_y = y + dirs[dir as usize][1];
    if map.contains_key(&(next_x, next_y)) {
        walk_step_count(map, dir, next_x, next_y, count);
    } else {
        for (new_dir, xy) in dirs.iter().enumerate() {
            if new_dir == ((dir + 2) % 4) as usize || new_dir == dir as usize {
                continue;
            }
            let next_x = x + xy[0];
            let next_y = y + xy[1];
            if map.contains_key(&(next_x, next_y)) {
                walk_step_count(map, new_dir as i32, next_x, next_y, count);
            }
        }
    }
}
