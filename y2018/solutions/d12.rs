use std::collections::{HashMap, HashSet};

use utils::{file_reader, harness::Solve};

pub struct D12;

impl Solve for D12 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut pots: HashSet<i32> = HashSet::new();
        let mut rules: HashMap<String, String> = HashMap::new();
        for (i, input) in inputs.iter().enumerate() {
            if i == 0 {
                let state = input.split(" ").last().unwrap();
                for (j, c) in state.chars().enumerate() {
                    if c == '#' {
                        pots.insert(j as i32);
                    }
                }
                continue;
            }
            if i == 1 {
                continue;
            }
            let words: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
            rules.insert(words[0].clone(), words[2].clone());
        }

        for _ in 0..20 {
            let start = *pots.iter().min().unwrap();
            let end = *pots.iter().max().unwrap();
            let mut temp: HashSet<i32> = HashSet::new();
            for i in start - 3..=end + 3 {
                let mut key = "".to_string();
                for j in i - 2..=i + 2 {
                    if pots.contains(&j) {
                        key.push('#')
                    } else {
                        key.push('.');
                    }
                }
                if rules.contains_key(&key) {
                    let res = rules[&key].clone();
                    if res == "#".to_string() {
                        temp.insert(i);
                    }
                }
            }
            pots = temp;
        }

        return pots.iter().sum::<i32>().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut pots: HashSet<i32> = HashSet::new();
        let mut rules: HashMap<String, String> = HashMap::new();
        for (i, input) in inputs.iter().enumerate() {
            if i == 0 {
                let state = input.split(" ").last().unwrap();
                for (j, c) in state.chars().enumerate() {
                    if c == '#' {
                        pots.insert(j as i32);
                    }
                }
                continue;
            }
            if i == 1 {
                continue;
            }
            let words: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
            rules.insert(words[0].clone(), words[2].clone());
        }

        for _ in 0..102 {
            let start = *pots.iter().min().unwrap();
            let end = *pots.iter().max().unwrap();
            let mut temp: HashSet<i32> = HashSet::new();
            for i in start - 3..=end + 3 {
                let mut key = "".to_string();
                for j in i - 2..=i + 2 {
                    if pots.contains(&j) {
                        key.push('#')
                    } else {
                        key.push('.');
                    }
                }
                if rules.contains_key(&key) {
                    let res = rules[&key].clone();
                    if res == "#".to_string() {
                        temp.insert(i);
                    }
                }
            }
            pots = temp;
        }

        let val = pots.iter().sum::<i32>();

        return (69 * (50000000000 - 102) + val as i64).to_string();
    }
}
