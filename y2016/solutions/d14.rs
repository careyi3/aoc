use fancy_regex::Regex;
use md5::compute;
use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D14;

impl Solve for D14 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let secret = input.first().unwrap();

        let three = Regex::new(r"(.)\1{2,}").unwrap();
        let five = Regex::new(r"(.)\1{4,}").unwrap();

        let mut threes: HashMap<i32, char> = HashMap::new();
        let mut fives: HashMap<char, Vec<i32>> = HashMap::new();

        let mut digest;
        for i in 0..22000 {
            let message = format!("{}{}", secret, i);
            digest = compute(message);
            let hex_digest = format!("{:x}", digest);

            if three.is_match(hex_digest.as_str()).unwrap() {
                let caps = three.captures(hex_digest.as_str()).unwrap().unwrap();
                let mat = caps.get(0).unwrap();
                let char = mat.as_str().chars().next().unwrap();
                threes.insert(i, char);
            }

            if five.is_match(hex_digest.as_str()).unwrap() {
                let caps = five.captures(hex_digest.as_str()).unwrap().unwrap();
                let mat = caps.get(0).unwrap();
                let char = mat.as_str().chars().next().unwrap();
                fives
                    .entry(char)
                    .and_modify(|x| x.push(i))
                    .or_insert(vec![i]);
            }
        }

        let mut keys = 0;
        let mut ans = 0;
        let mut sorted: Vec<&i32> = threes.keys().collect();
        sorted.sort();
        'outer: for id in sorted {
            let c = threes.get(&id).unwrap();
            if fives.contains_key(&c) {
                let five_ids = fives.get(&c).unwrap();
                for five_id in five_ids {
                    if *five_id > *id && *five_id - *id < 1000 {
                        keys += 1;
                        if keys == 64 {
                            ans = *id;
                        }
                        continue 'outer;
                    }
                }
            }
        }

        return ans.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let secret = input.first().unwrap();

        let three = Regex::new(r"(.)\1{2,}").unwrap();
        let five = Regex::new(r"(.)\1{4,}").unwrap();

        let mut threes: HashMap<i32, char> = HashMap::new();
        let mut fives: HashMap<char, Vec<i32>> = HashMap::new();

        for i in 0..22000 {
            let message = format!("{}{}", secret, i);
            let hex_digest = hash(message);

            if three.is_match(hex_digest.as_str()).unwrap() {
                let caps = three.captures(hex_digest.as_str()).unwrap().unwrap();
                let mat = caps.get(0).unwrap();
                let char = mat.as_str().chars().next().unwrap();
                threes.insert(i, char);
            }

            if five.is_match(hex_digest.as_str()).unwrap() {
                let caps = five.captures(hex_digest.as_str()).unwrap().unwrap();
                let mat = caps.get(0).unwrap();
                let char = mat.as_str().chars().next().unwrap();
                fives
                    .entry(char)
                    .and_modify(|x| x.push(i))
                    .or_insert(vec![i]);
            }
        }

        let mut keys = 0;
        let mut ans = 0;
        let mut sorted: Vec<&i32> = threes.keys().collect();
        sorted.sort();
        'outer: for id in sorted {
            let c = threes.get(&id).unwrap();
            if fives.contains_key(&c) {
                let five_ids = fives.get(&c).unwrap();
                for five_id in five_ids {
                    if *five_id > *id && *five_id - *id < 1000 {
                        keys += 1;
                        if keys == 64 {
                            ans = *id;
                        }
                        continue 'outer;
                    }
                }
            }
        }

        return ans.to_string();
    }
}

fn hash(mut message: String) -> String {
    for _ in 0..2017 {
        let digest = compute(message);
        message = format!("{:x}", digest);
    }
    return message;
}
