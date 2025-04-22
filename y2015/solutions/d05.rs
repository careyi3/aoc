use fancy_regex::Regex;
use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D05;

impl Solve for D05 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut sum = 0;
        'outer: for input in inputs {
            let mut vowles: HashMap<char, i32> =
                HashMap::from([('a', 0), ('e', 0), ('i', 0), ('o', 0), ('u', 0)]);
            let mut doubles: HashMap<String, i32> = HashMap::new();

            let chars: Vec<char> = input.chars().collect();
            let length = input.chars().count();
            for (i, char) in input.chars().enumerate() {
                vowles.entry(char).and_modify(|counter| *counter += 1);
                if i < length - 1 {
                    let key = format!("{}{}", chars[i], chars[i + 1]);
                    doubles
                        .entry(key)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
            }

            if vowles.values().sum::<i32>() < 3 {
                continue;
            }

            if doubles.contains_key("ab") {
                continue;
            }
            if doubles.contains_key("cd") {
                continue;
            }
            if doubles.contains_key("pq") {
                continue;
            }
            if doubles.contains_key("xy") {
                continue;
            }

            for i in 97u8..123u8 {
                let key = format!("{}{}", i as char, i as char);
                if doubles.contains_key(&key) {
                    sum += 1;
                    continue 'outer;
                }
            }
        }

        return sum.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let match_1 = Regex::new(r"(..).*\1").unwrap();
        let match_2 = Regex::new(r"(.).\1").unwrap();

        let mut sum = 0;
        for input in inputs {
            if match_1.is_match(&input).unwrap() && match_2.is_match(&input).unwrap() {
                sum += 1;
            }
        }

        return sum.to_string();
    }
}
