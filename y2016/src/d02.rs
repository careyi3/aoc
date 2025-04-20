use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D02;

impl Solve for D02 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut x = 0;
        let mut y = 0;
        let moves = HashMap::from([
            ("L".to_string(), 0),
            ("U".to_string(), 1),
            ("R".to_string(), 2),
            ("D".to_string(), 3),
        ]);
        let coords = HashMap::from([
            ((-1, 1), 1),
            ((0, 1), 2),
            ((1, 1), 3),
            ((-1, 0), 4),
            ((0, 0), 5),
            ((1, 0), 6),
            ((-1, -1), 7),
            ((0, -1), 8),
            ((1, -1), 9),
        ]);
        let dirs = [[-1, 0], [0, 1], [1, 0], [0, -1]];

        let mut code: Vec<String> = vec![];
        for input in inputs {
            for char in input.chars().map(|x| x.to_string()) {
                let move_idx = moves[&char];
                let x_move = dirs[move_idx][0];
                let y_move = dirs[move_idx][1];

                let new_x = x + x_move;
                if new_x <= 1 && new_x >= -1 {
                    x = new_x;
                }

                let new_y = y + y_move;
                if new_y <= 1 && new_y >= -1 {
                    y = new_y;
                }
            }
            let val = coords[&(x, y)].to_string();
            code.push(val);
        }

        return code.join("").to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut x = -2;
        let mut y = 0;
        let moves = HashMap::from([
            ("L".to_string(), 0),
            ("U".to_string(), 1),
            ("R".to_string(), 2),
            ("D".to_string(), 3),
        ]);
        let coords = HashMap::from([
            ((0, 2), "1".to_string()),
            ((-1, 1), "2".to_string()),
            ((0, 1), "3".to_string()),
            ((1, 1), "4".to_string()),
            ((-2, 0), "5".to_string()),
            ((-1, 0), "6".to_string()),
            ((0, 0), "7".to_string()),
            ((1, 0), "8".to_string()),
            ((2, 0), "9".to_string()),
            ((-1, -1), "A".to_string()),
            ((0, -1), "B".to_string()),
            ((1, -1), "C".to_string()),
            ((0, -2), "D".to_string()),
        ]);
        let dirs = [[-1, 0], [0, 1], [1, 0], [0, -1]];

        let mut code: Vec<String> = vec![];
        for input in inputs {
            for char in input.chars().map(|x| x.to_string()) {
                let move_idx = moves[&char];
                let x_move = dirs[move_idx][0];
                let y_move = dirs[move_idx][1];

                let new_x = x + x_move;
                if coords.contains_key(&(new_x, y)) {
                    x = new_x;
                }

                let new_y = y + y_move;
                if coords.contains_key(&(x, new_y)) {
                    y = new_y;
                }
            }
            let val = coords[&(x, y)].to_string();
            code.push(val);
        }

        return code.join("").to_string();
    }
}
