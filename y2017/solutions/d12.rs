use std::collections::{HashMap, HashSet};

use utils::{file_reader, harness::Solve};

pub struct D12;

impl Solve for D12 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
        for input in inputs {
            let tokens: Vec<String> = input.split(" <-> ").map(|x| x.to_string()).collect();
            let left: i32 = tokens.first().unwrap().parse().unwrap();
            for right in tokens
                .last()
                .unwrap()
                .split(", ")
                .map(|x| x.parse::<i32>().unwrap())
            {
                map.entry(left)
                    .and_modify(|x| {
                        x.insert(right);
                        return;
                    })
                    .or_insert(HashSet::from([right]));
                map.entry(right)
                    .and_modify(|x| {
                        x.insert(left);
                        return;
                    })
                    .or_insert(HashSet::from([left]));
            }
        }

        let mut visited: HashSet<i32> = HashSet::new();

        walk(&map, &mut visited, 0);

        return visited.len().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut to_visit: HashSet<i32> = HashSet::new();
        for input in inputs {
            let tokens: Vec<String> = input.split(" <-> ").map(|x| x.to_string()).collect();
            let left: i32 = tokens.first().unwrap().parse().unwrap();
            to_visit.insert(left);
            for right in tokens
                .last()
                .unwrap()
                .split(", ")
                .map(|x| x.parse::<i32>().unwrap())
            {
                to_visit.insert(right);
                map.entry(left)
                    .and_modify(|x| {
                        x.insert(right);
                        return;
                    })
                    .or_insert(HashSet::from([right]));
                map.entry(right)
                    .and_modify(|x| {
                        x.insert(left);
                        return;
                    })
                    .or_insert(HashSet::from([left]));
            }
        }

        let mut group_count = 0;
        let mut next_id = 0;
        while to_visit.len() > 0 {
            let mut visited: HashSet<i32> = HashSet::new();

            walk_to_visit(&map, &mut visited, &mut to_visit, next_id);

            group_count += 1;

            if to_visit.len() > 0 {
                next_id = to_visit.iter().nth(0).unwrap().clone();
            }
        }

        return group_count.to_string();
    }
}

fn walk(map: &HashMap<i32, HashSet<i32>>, visited: &mut HashSet<i32>, id: i32) {
    if visited.contains(&id) {
        return;
    }
    visited.insert(id);
    let neighbours = map[&id].clone();
    for n in neighbours {
        walk(map, visited, n);
    }
}

fn walk_to_visit(
    map: &HashMap<i32, HashSet<i32>>,
    visited: &mut HashSet<i32>,
    to_visit: &mut HashSet<i32>,
    id: i32,
) {
    if visited.contains(&id) {
        return;
    }
    visited.insert(id);
    to_visit.remove(&id);
    let neighbours = map[&id].clone();
    for n in neighbours {
        walk_to_visit(map, visited, to_visit, n);
    }
}
