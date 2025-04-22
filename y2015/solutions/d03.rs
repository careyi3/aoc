use std::collections::HashMap;

use utils::file_reader::read_lines;
use utils::harness::Solve;

pub struct D03;

impl Solve for D03 {
    fn part1(_input: String, path: &String) -> String {
        let mut x = 0;
        let mut y = 0;
        let mut map = HashMap::from([((0, 0), 1)]);
        for instruction in read_lines(path).first().unwrap().chars() {
            let dir = dir(instruction);
            x += dir[0];
            y += dir[1];

            map.entry((x, y))
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        return map.keys().count().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let mut santa_x = 0;
        let mut santa_y = 0;
        let mut robo_santa_x = 0;
        let mut robo_santa_y = 0;
        let mut map = HashMap::from([((0, 0), 1)]);
        for (i, instruction) in read_lines(path).first().unwrap().chars().enumerate() {
            let dir = dir(instruction);

            let x_position;
            let y_position;
            if i % 2 == 0 {
                robo_santa_x += dir[0];
                robo_santa_y += dir[1];
                x_position = robo_santa_x;
                y_position = robo_santa_y;
            } else {
                santa_x += dir[0];
                santa_y += dir[1];
                x_position = santa_x;
                y_position = santa_y;
            }

            map.entry((x_position, y_position))
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        return map.keys().count().to_string();
    }
}

fn dir(symbol: char) -> [i32; 2] {
    let lookup = HashMap::from([('<', 0), ('^', 1), ('>', 2), ('v', 3)]);
    let idx = lookup.get(&symbol).unwrap();
    return dirs()[*idx];
}

fn dirs() -> [[i32; 2]; 4] {
    return [[-1, 0], [0, 1], [1, 0], [0, -1]];
}
