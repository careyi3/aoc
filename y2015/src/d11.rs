use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D11;

impl Solve for D11 {
    fn part1(_input: String, path: &String) -> String {
        let mut password: Vec<char> = file_reader::read_lines(path)
            .first()
            .unwrap()
            .chars()
            .collect();

        loop {
            password = increment(password, 7);
            if valid(&password) {
                break;
            }
        }

        return password.into_iter().collect::<String>();
    }

    fn part2(_input: String, path: &String) -> String {
        let mut password: Vec<char> = file_reader::read_lines(path)
            .first()
            .unwrap()
            .chars()
            .collect();

        let mut count = 0;
        loop {
            password = increment(password, 7);
            if valid(&password) {
                count += 1;
                if count == 2 {
                    break;
                }
            }
        }

        return password.into_iter().collect::<String>();
    }
}

fn valid(password: &Vec<char>) -> bool {
    if password.contains(&'i') {
        return false;
    }
    if password.contains(&'o') {
        return false;
    }
    if password.contains(&'l') {
        return false;
    }

    if !contains_pairs(&password) {
        return false;
    }
    if !contains_straight(&password) {
        return false;
    }

    return true;
}

fn contains_pairs(password: &Vec<char>) -> bool {
    let mut occurrences: HashMap<char, Vec<i32>> = HashMap::new();

    for i in 0..8 {
        let key = password[i];
        occurrences
            .entry(key)
            .and_modify(|x| x.push(i.try_into().unwrap()))
            .or_insert(vec![i.try_into().unwrap()]);
    }

    let mut matches = 2;

    for seq in occurrences.values() {
        if seq.len() < 2 {
            continue;
        } else {
            let l = seq.len();
            let mut sorted = seq.clone();
            sorted.sort();
            for i in 0..l - 1 {
                let one = sorted[i];
                let two = sorted[i + 1];
                if one + 1 == two {
                    matches -= 1;
                    break;
                }
            }
        }
    }

    return matches == 0;
}

fn contains_straight(password: &Vec<char>) -> bool {
    for i in 0..6 {
        let one = password[i] as u8;
        let two = password[i + 1] as u8;
        let three = password[i + 2] as u8;

        if one + 1 == two && three - 1 == two {
            return true;
        }
    }

    return false;
}

fn increment(mut password: Vec<char>, idx: usize) -> Vec<char> {
    if password[idx] == 'z' {
        password[idx] = 'a';
        return increment(password, idx - 1);
    } else {
        let temp = password[idx] as u8;
        password[idx] = (temp + 1) as char;
    }

    return password;
}
