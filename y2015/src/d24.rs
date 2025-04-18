use itertools::Itertools;
use std::collections::HashSet;
use utils::{file_reader, harness::Solve};

pub struct D24;

impl Solve for D24 {
    fn part1(_input: String, path: &String) -> String {
        let inputs: Vec<i32> = file_reader::read_lines(path)
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();
        let nums: HashSet<i32> = HashSet::from_iter(inputs);

        let sum: i32 = nums.iter().sum();
        let target = sum / 3;

        let mut results: Vec<HashSet<i32>> = vec![];
        let with: HashSet<i32> = HashSet::new();
        let mut cache: HashSet<String> = HashSet::new();
        find(target, with, nums, &mut 28, &mut cache, &mut results);

        let mut ents: Vec<i64> = vec![];
        for result in results {
            let mut ent: i64 = 1;
            for n in result {
                ent *= n as i64;
            }
            ents.push(ent);
        }

        return ents.iter().min().unwrap().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs: Vec<i32> = file_reader::read_lines(path)
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();
        let nums: HashSet<i32> = HashSet::from_iter(inputs);

        let sum: i32 = nums.iter().sum();
        let target = sum / 4;

        let mut results: Vec<HashSet<i32>> = vec![];
        let with: HashSet<i32> = HashSet::new();
        let mut cache: HashSet<String> = HashSet::new();
        find(target, with, nums, &mut 6, &mut cache, &mut results);

        let mut ents: Vec<i64> = vec![];
        for result in results {
            let mut ent: i64 = 1;
            for n in result {
                ent *= n as i64;
            }
            ents.push(ent);
        }

        return ents.iter().min().unwrap().to_string();
    }
}

fn find(
    target: i32,
    with: HashSet<i32>,
    from: HashSet<i32>,
    shortest: &mut i32,
    cache: &mut HashSet<String>,
    results: &mut Vec<HashSet<i32>>,
) {
    let key = format!(
        "{}:{}",
        with.iter().sorted().join(","),
        from.iter().sorted().join(",")
    );
    if cache.contains(&key) {
        return;
    } else {
        cache.insert(key);
    }

    let len = with.len() as i32;
    if len > *shortest {
        return;
    }

    let with_sum: i32 = with.iter().sum();
    if with_sum == target {
        let len = with.len() as i32;
        if len < *shortest {
            *shortest = len;
        }
        results.push(with);
        return;
    }
    if with_sum > 516 {
        return;
    }

    let temp_from = from.clone();
    for num in from {
        let mut new_with = with.clone();
        new_with.insert(num);
        let mut new_from = temp_from.clone();
        new_from.remove(&num);

        find(target, new_with, new_from, shortest, cache, results);
    }
}
