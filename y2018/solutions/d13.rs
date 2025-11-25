use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D13;

impl Solve for D13 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        const DIRS: [[i32; 2]; 4] = [[-1, 0], [0, -1], [1, 0], [0, 1]];

        let mut map: HashMap<(i32, i32), [[i32; 4]; 4]> = HashMap::new();
        let mut carts: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut y = 0;
        for line in input {
            let mut x = 0;
            for char in line.chars() {
                match char {
                    '>' => {
                        carts.insert((x, y), (2, 0));
                        map.insert(
                            (x, y),
                            [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
                        );
                    }
                    '<' => {
                        carts.insert((x, y), (0, 0));
                        map.insert(
                            (x, y),
                            [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
                        );
                    }
                    '^' => {
                        carts.insert((x, y), (1, 0));
                        map.insert(
                            (x, y),
                            [[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]],
                        );
                    }
                    'v' => {
                        carts.insert((x, y), (3, 0));
                        map.insert(
                            (x, y),
                            [[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]],
                        );
                    }
                    '/' => {
                        map.insert(
                            (x, y),
                            [[0, 0, 0, 1], [0, 0, 1, 0], [0, 1, 0, 0], [1, 0, 0, 0]],
                        );
                    }
                    '\\' => {
                        map.insert(
                            (x, y),
                            [[0, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 1], [0, 0, 1, 0]],
                        );
                    }
                    '|' => {
                        map.insert(
                            (x, y),
                            [[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]],
                        );
                    }
                    '-' => {
                        map.insert(
                            (x, y),
                            [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
                        );
                    }
                    '+' => {
                        map.insert(
                            (x, y),
                            [[0, 1, 1, 1], [1, 0, 1, 1], [1, 1, 0, 1], [1, 1, 1, 0]],
                        );
                    }
                    _ => {}
                }
                x += 1;
            }
            y += 1;
        }

        let ans_x;
        let ans_y;
        'outer: loop {
            let mut new_carts: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

            let mut coords: Vec<_> = carts.keys().copied().collect();
            coords.sort_by_key(|&(x, y)| (y, x));

            for (x, y) in coords {
                let (dir, turn) = carts[&(x, y)];
                carts.remove(&(x, y));
                let dirs = map[&(x, y)][dir as usize];
                let mut new_turn = turn;
                let mut new_dir = dir;
                if dirs.iter().sum::<i32>() > 1 {
                    match turn {
                        0 => new_dir = (dir - 1).rem_euclid(4),
                        2 => new_dir = (dir + 1).rem_euclid(4),
                        _ => {}
                    }
                    new_turn = (turn + 1) % 3;
                } else {
                    for (i, d) in dirs.iter().enumerate() {
                        if *d == 1 {
                            new_dir = i as i32;
                        }
                    }
                }
                let new_x = x + DIRS[new_dir as usize][0];
                let new_y = y + DIRS[new_dir as usize][1];
                if new_carts.contains_key(&(new_x, new_y)) || carts.contains_key(&(new_x, new_y)) {
                    ans_x = new_x;
                    ans_y = new_y;
                    break 'outer;
                } else {
                    new_carts.insert((new_x, new_y), (new_dir, new_turn));
                }
            }
            carts = new_carts;
        }

        return format!("{},{}", ans_x, ans_y);
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        const DIRS: [[i32; 2]; 4] = [[-1, 0], [0, -1], [1, 0], [0, 1]];

        let mut map: HashMap<(i32, i32), [[i32; 4]; 4]> = HashMap::new();
        let mut carts: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut y = 0;
        for line in input {
            let mut x = 0;
            for char in line.chars() {
                match char {
                    '>' => {
                        carts.insert((x, y), (2, 0));
                        map.insert(
                            (x, y),
                            [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
                        );
                    }
                    '<' => {
                        carts.insert((x, y), (0, 0));
                        map.insert(
                            (x, y),
                            [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
                        );
                    }
                    '^' => {
                        carts.insert((x, y), (1, 0));
                        map.insert(
                            (x, y),
                            [[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]],
                        );
                    }
                    'v' => {
                        carts.insert((x, y), (3, 0));
                        map.insert(
                            (x, y),
                            [[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]],
                        );
                    }
                    '/' => {
                        map.insert(
                            (x, y),
                            [[0, 0, 0, 1], [0, 0, 1, 0], [0, 1, 0, 0], [1, 0, 0, 0]],
                        );
                    }
                    '\\' => {
                        map.insert(
                            (x, y),
                            [[0, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 1], [0, 0, 1, 0]],
                        );
                    }
                    '|' => {
                        map.insert(
                            (x, y),
                            [[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]],
                        );
                    }
                    '-' => {
                        map.insert(
                            (x, y),
                            [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
                        );
                    }
                    '+' => {
                        map.insert(
                            (x, y),
                            [[0, 1, 1, 1], [1, 0, 1, 1], [1, 1, 0, 1], [1, 1, 1, 0]],
                        );
                    }
                    _ => {}
                }
                x += 1;
            }
            y += 1;
        }

        let ans_x;
        let ans_y;
        'outer: loop {
            let mut new_carts: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

            let mut coords: Vec<_> = carts.keys().copied().collect();
            coords.sort_by_key(|&(x, y)| (y, x));

            for (x, y) in coords {
                if !carts.contains_key(&(x, y)) {
                    continue;
                }
                let (dir, turn) = carts[&(x, y)];
                carts.remove(&(x, y));
                let dirs = map[&(x, y)][dir as usize];
                let mut new_turn = turn;
                let mut new_dir = dir;
                if dirs.iter().sum::<i32>() > 1 {
                    match turn {
                        0 => new_dir = (dir - 1).rem_euclid(4),
                        2 => new_dir = (dir + 1).rem_euclid(4),
                        _ => {}
                    }
                    new_turn = (turn + 1) % 3;
                } else {
                    for (i, d) in dirs.iter().enumerate() {
                        if *d == 1 {
                            new_dir = i as i32;
                        }
                    }
                }
                let new_x = x + DIRS[new_dir as usize][0];
                let new_y = y + DIRS[new_dir as usize][1];
                if new_carts.contains_key(&(new_x, new_y)) || carts.contains_key(&(new_x, new_y)) {
                    if new_carts.contains_key(&(new_x, new_y)) {
                        new_carts.remove(&(new_x, new_y));
                    }
                    if carts.contains_key(&(new_x, new_y)) {
                        carts.remove(&(new_x, new_y));
                    }
                    if carts.len() + new_carts.len() == 1 {
                        if carts.len() == 1 {
                            let (coords, _) = carts.iter().next().unwrap();
                            ans_x = coords.0;
                            ans_y = coords.1;
                        } else {
                            let (coords, _) = new_carts.iter().next().unwrap();
                            ans_x = coords.0;
                            ans_y = coords.1;
                        }
                        break 'outer;
                    }
                } else {
                    new_carts.insert((new_x, new_y), (new_dir, new_turn));
                }
            }
            carts = new_carts;
        }

        return format!("{},{}", ans_x, ans_y);
    }
}
