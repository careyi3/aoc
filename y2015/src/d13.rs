use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use utils::{file_reader, harness::Solve};

pub struct D13;

impl Solve for D13 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut lookup: HashMap<(String, String), i32> = HashMap::new();
        let mut people: HashSet<String> = HashSet::new();

        for input in inputs {
            let words: Vec<&str> = input.split(' ').collect();
            let a = words[0].to_string();
            let b = words[10].to_string();
            people.insert(a.clone());
            people.insert(b.clone());
            let op = words[2];
            let amount: i32 = words[3].parse().unwrap();
            let mut pairing = vec![a, b];
            pairing.sort();
            let key = (pairing[0].clone(), pairing[1].clone());

            if op == "gain" {
                lookup
                    .entry(key)
                    .and_modify(|x| *x += amount)
                    .or_insert(amount);
            } else {
                lookup
                    .entry(key)
                    .and_modify(|x| *x -= amount)
                    .or_insert(-amount);
            }
        }

        let mut highest = 0;
        let guest_count = people.len();
        for persons in people.into_iter().permutations(guest_count) {
            let mut sum = 0;
            for i in 0..guest_count {
                let mut pairing;
                if i == guest_count - 1 {
                    pairing = vec![persons[0].clone(), persons[guest_count - 1].clone()];
                } else {
                    pairing = vec![persons[i].clone(), persons[i + 1].clone()];
                }
                pairing.sort();
                let key = (pairing[0].clone(), pairing[1].clone());
                sum += lookup[&key]
            }
            if sum > highest {
                highest = sum;
            }
            // println!("{}:{}", persons.join(","), sum);
        }

        return highest.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut lookup: HashMap<(String, String), i32> = HashMap::new();
        let mut people: HashSet<String> = HashSet::new();

        for input in inputs {
            let words: Vec<&str> = input.split(' ').collect();
            let a = words[0].to_string();
            let b = words[10].to_string();
            people.insert(a.clone());
            people.insert(b.clone());
            let op = words[2];
            let amount: i32 = words[3].parse().unwrap();
            let mut pairing = vec![a, b];
            pairing.sort();
            let key = (pairing[0].clone(), pairing[1].clone());

            if op == "gain" {
                lookup
                    .entry(key)
                    .and_modify(|x| *x += amount)
                    .or_insert(amount);
            } else {
                lookup
                    .entry(key)
                    .and_modify(|x| *x -= amount)
                    .or_insert(-amount);
            }
        }

        let mut highest = 0;
        let guest_count = people.len();
        for persons in people.into_iter().permutations(guest_count) {
            let mut sum = 0;
            for i in 0..guest_count {
                let mut pairing;
                if i == guest_count - 1 {
                    pairing = vec![persons[0].clone(), persons[guest_count - 1].clone()];
                } else {
                    pairing = vec![persons[i].clone(), persons[i + 1].clone()];
                }
                pairing.sort();
                let key = (pairing[0].clone(), pairing[1].clone());
                sum += lookup[&key]
            }
            if sum > highest {
                highest = sum;
            }
            // println!("{}:{}", persons.join(","), sum);
        }

        return highest.to_string();
    }
}
