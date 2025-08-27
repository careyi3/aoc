use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D05;

impl Solve for D05 {
    fn part1(_input: String, path: &String) -> String {
        let input = &file_reader::read_lines(path)[0];

        let mut polymer: Vec<char> = input.clone().chars().collect();
        let mut reduced = true;
        while reduced {
            reduced = false;
            let mut temp = vec![];
            let mut i = 0;
            while i < polymer.len() - 1 {
                let a = polymer[i];
                let b = polymer[i + 1];
                if reacts(a, b) {
                    i += 2;
                    reduced = true;
                }
                temp.push(polymer[i]);
                if i + 1 == polymer.len() - 1 {
                    temp.push(polymer[i + 1]);
                }
                i += 1;
            }
            polymer = temp;
        }

        return polymer.len().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = &file_reader::read_lines(path)[0];

        let mut map: HashMap<char, usize> = HashMap::new();

        for c in 'a'..='z' {
            let mut polymer: Vec<char> = input
                .clone()
                .chars()
                .filter(|x| !x.eq_ignore_ascii_case(&c))
                .collect();
            let mut reduced = true;
            while reduced {
                reduced = false;
                let mut temp = vec![];
                let mut i = 0;
                while i < polymer.len() - 1 {
                    let a = polymer[i];
                    let b = polymer[i + 1];
                    if reacts(a, b) {
                        i += 2;
                        reduced = true;
                    }
                    temp.push(polymer[i]);
                    if i + 1 == polymer.len() - 1 {
                        temp.push(polymer[i + 1]);
                    }
                    i += 1;
                }
                polymer = temp;
            }

            map.insert(c, polymer.len());
        }

        return map.values().min().unwrap().to_string();
    }
}

fn reacts(a: char, b: char) -> bool {
    a != b && a.eq_ignore_ascii_case(&b)
}
