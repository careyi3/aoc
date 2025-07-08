use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D11;

impl Solve for D11 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let path: Vec<String> = input
            .first()
            .unwrap()
            .split(',')
            .map(|x| x.to_string())
            .collect();
        // q, s, r
        let mut current_coord = (0, 0, 0);
        for step in path {
            let dir = dirs(step);
            current_coord = (
                current_coord.0 + dir.0,
                current_coord.1 + dir.1,
                current_coord.2 + dir.2,
            );
        }

        let dist = (current_coord.0.abs() + current_coord.1.abs() + current_coord.2.abs()) / 2;

        return dist.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let path: Vec<String> = input
            .first()
            .unwrap()
            .split(',')
            .map(|x| x.to_string())
            .collect();
        let mut highest = 0;
        // q, s, r
        let mut current_coord = (0, 0, 0);
        for step in path {
            let dir = dirs(step);
            current_coord = (
                current_coord.0 + dir.0,
                current_coord.1 + dir.1,
                current_coord.2 + dir.2,
            );
            let dist = (current_coord.0.abs() + current_coord.1.abs() + current_coord.2.abs()) / 2;
            if dist > highest {
                highest = dist;
            }
        }

        return highest.to_string();
    }
}

fn dirs(dir: String) -> (i32, i32, i32) {
    let dirs = HashMap::from([
        ("n".to_string(), (0, 1, -1)),
        ("s".to_string(), (0, -1, 1)),
        ("ne".to_string(), (1, 0, -1)),
        ("sw".to_string(), (-1, 0, 1)),
        ("nw".to_string(), (-1, 1, 0)),
        ("se".to_string(), (1, -1, 0)),
    ]);
    return dirs[&dir];
}
