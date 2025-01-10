use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use utils::{file_reader, harness::Solve};

pub struct D09;

impl Solve for D09 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut places: HashSet<String> = HashSet::new();
        let mut distances: HashMap<String, i32> = HashMap::new();

        for input in inputs {
            let tokens: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();
            let hash_key = key(&tokens[0], &tokens[2]);
            distances.insert(hash_key, tokens[4].parse().unwrap());
            places.insert(tokens[0].clone());
            places.insert(tokens[2].clone());
        }

        let num_places = places.len();
        let mut total_distances: Vec<i32> = vec![];
        'perm: for per in places.into_iter().permutations(num_places) {
            let mut dist = 0;

            for i in 0..num_places - 1 {
                let key = key(&per[i], &per[i + 1]);

                if distances.contains_key(&key) {
                    dist += distances[&key];
                } else {
                    continue 'perm;
                }
            }
            total_distances.push(dist);
        }

        return total_distances.iter().min().unwrap().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut places: HashSet<String> = HashSet::new();
        let mut distances: HashMap<String, i32> = HashMap::new();

        for input in inputs {
            let tokens: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();
            let hash_key = key(&tokens[0], &tokens[2]);
            distances.insert(hash_key, tokens[4].parse().unwrap());
            places.insert(tokens[0].clone());
            places.insert(tokens[2].clone());
        }

        let num_places = places.len();
        let mut total_distances: Vec<i32> = vec![];
        'perm: for per in places.into_iter().permutations(num_places) {
            let mut dist = 0;

            for i in 0..num_places - 1 {
                let key = key(&per[i], &per[i + 1]);

                if distances.contains_key(&key) {
                    dist += distances[&key];
                } else {
                    continue 'perm;
                }
            }
            total_distances.push(dist);
        }

        return total_distances.iter().max().unwrap().to_string();
    }
}

fn key(a: &String, b: &String) -> String {
    let mut locations = [a, b];
    locations.sort();
    return format!("{}:{}", locations[0], locations[1]);
}
