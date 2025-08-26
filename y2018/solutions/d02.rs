use itertools::Itertools;
use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

use edit_distance::edit_distance;

pub struct D02;

impl Solve for D02 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut twos = 0;
        let mut threes = 0;
        for input in inputs {
            let mut counts: HashMap<char, i32> = HashMap::new();
            for c in input.chars() {
                counts.entry(c).and_modify(|x| *x += 1).or_insert(1);
            }

            let vals: Vec<i32> = counts.values().into_iter().map(|x| *x).collect();

            for val in vals.into_iter().unique() {
                if val == 2 {
                    twos += 1;
                }
                if val == 3 {
                    threes += 1;
                }
            }
        }

        return (twos * threes).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut ans: String = "".to_string();
        'outer: for x in &inputs {
            for y in &inputs {
                if x == y {
                    continue;
                }
                if edit_distance(x, y) == 1 {
                    let x_chars: Vec<char> = x.chars().collect();
                    let y_chars: Vec<char> = y.chars().collect();
                    for i in 0..x_chars.len() {
                        if x_chars[i] == y_chars[i] {
                            ans.push(x_chars[i]);
                        }
                    }
                    break 'outer;
                }
            }
        }

        return ans;
    }
}
