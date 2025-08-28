use std::collections::{HashMap, HashSet};

use utils::{file_reader, harness::Solve};

pub struct D07;

impl Solve for D07 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut deps: HashMap<String, Vec<String>> = HashMap::new();
        let mut tos: HashSet<String> = HashSet::new();
        let mut froms: HashSet<String> = HashSet::new();
        let mut to_do: HashSet<String> = HashSet::new();
        for input in inputs {
            let words: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
            let from = words[1].clone();
            let to = words[7].clone();
            froms.insert(from.clone());
            tos.insert(to.clone());
            to_do.insert(to.clone());
            to_do.insert(from.clone());
            map.entry(from.clone())
                .and_modify(|x| x.push(to.clone()))
                .or_insert(vec![to.clone()]);
            deps.entry(to.clone())
                .and_modify(|x| x.push(from.clone()))
                .or_insert(vec![from.clone()]);
        }

        let mut available = froms
            .difference(&tos)
            .map(|x| x.clone())
            .collect::<Vec<String>>();
        available.sort();
        available.reverse();
        let mut path = "".to_string();
        while available.len() > 0 {
            let i = available.pop().unwrap();
            to_do.remove(&i);
            path.push_str(&i);
            if map.contains_key(&i) {
                for a in map[&i].clone() {
                    let mut is_available = true;
                    for dep in deps[&a].clone() {
                        if to_do.contains(&dep) {
                            is_available = false;
                        }
                    }
                    if is_available {
                        available.push(a);
                    }
                }
            }
            available.sort();
            available.reverse();
        }

        return path;
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut deps: HashMap<String, Vec<String>> = HashMap::new();
        let mut tos: HashSet<String> = HashSet::new();
        let mut froms: HashSet<String> = HashSet::new();
        let mut to_do: HashSet<String> = HashSet::new();
        for input in inputs {
            let words: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
            let from = words[1].clone();
            let to = words[7].clone();
            froms.insert(from.clone());
            tos.insert(to.clone());
            to_do.insert(to.clone());
            to_do.insert(from.clone());
            map.entry(from.clone())
                .and_modify(|x| x.push(to.clone()))
                .or_insert(vec![to.clone()]);
            deps.entry(to.clone())
                .and_modify(|x| x.push(from.clone()))
                .or_insert(vec![from.clone()]);
        }

        let mut available = froms
            .difference(&tos)
            .map(|x| x.clone())
            .collect::<Vec<String>>();
        available.sort();
        available.reverse();

        let mut workers: HashMap<String, i32> = HashMap::new();

        let mut path = "".to_string();
        let mut tick = 0;
        while available.len() > 0 {
            for i in available.clone() {
                if workers.contains_key(&i) {
                    if workers[&i] > 1 {
                        *workers.get_mut(&i).unwrap() -= 1;
                    } else {
                        available.retain(|x| x != &i);
                        to_do.remove(&i);
                        path.push_str(&i);
                        workers.remove(&i);
                        if map.contains_key(&i) {
                            for a in map[&i].clone() {
                                let mut is_available = true;
                                for dep in deps[&a].clone() {
                                    if to_do.contains(&dep) {
                                        is_available = false;
                                    }
                                }
                                if is_available {
                                    available.push(a.clone());
                                    if workers.len() < 5 {
                                        let b = a.as_bytes()[0].to_ascii_lowercase();
                                        let val = (b - b'a' + 1) as i32;
                                        workers.insert(a, val + 60);
                                    }
                                }
                            }
                        }
                        available.sort();
                    }
                } else {
                    if workers.len() < 5 {
                        let b = i.as_bytes()[0].to_ascii_lowercase();
                        let val = (b - b'a' + 1) as i32;
                        workers.insert(i, val + 60);
                    }
                }
            }
            tick += 1;
        }

        return (tick - 1).to_string();
    }
}
