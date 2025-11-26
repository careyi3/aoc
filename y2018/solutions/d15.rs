use anyhow::Result;
use utils::{file_reader, harness::SolveResult};

use std::collections::HashMap;

pub struct D15;

impl SolveResult for D15 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut units: HashMap<(i32, i32), (char, i32, i32)> = HashMap::new();
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'E' => {
                        units.insert((x as i32, y as i32), (c, 200, 3));
                    }
                    'G' => {
                        units.insert((x as i32, y as i32), (c, 200, 3));
                    }
                    _ => {}
                }
                map.insert((x as i32, y as i32), c);
            }
        }

        find_shortest_paths(map, vec![]);

        return Ok(input.first().unwrap().to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        return Ok(input.first().unwrap().to_string());
    }
}

fn find_shortest_paths(
    _map: HashMap<(i32, i32), char>,
    _targets: Vec<(i32, i32)>,
) -> Vec<Vec<(i32, i32)>> {
    find_paths();
    return vec![];
}

fn find_paths() {}
