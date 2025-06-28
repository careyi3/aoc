use std::collections::HashSet;

use itertools::Itertools;
use utils::{file_reader, harness::Solve};

pub struct D04;

impl Solve for D04 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut valid = 0;
        for input in inputs {
            let words: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
            let mut password: HashSet<String> = HashSet::new();
            valid += 1;
            for word in words {
                if password.contains(&word) {
                    valid -= 1;
                    break;
                } else {
                    password.insert(word);
                }
            }
        }

        return valid.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut valid = 0;
        for input in inputs {
            let words: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
            let mut password: HashSet<String> = HashSet::new();
            valid += 1;
            for word in words {
                if password.contains(&sorted(&word)) {
                    valid -= 1;
                    break;
                } else {
                    password.insert(sorted(&word));
                }
            }
        }

        return valid.to_string();
    }
}

fn sorted(input: &String) -> String {
    let mut chars = input.chars().sorted();
    return chars.join("").to_string();
}
