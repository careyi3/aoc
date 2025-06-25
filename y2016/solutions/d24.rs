use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D24;

impl Solve for D24 {
    fn part1(_input: String, path: &String) -> String {
        let lines = file_reader::read_lines(path);

        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let mut locs: HashMap<i32, (i32, i32)> = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char.to_digit(10) {
                    Some(num) => {
                        locs.insert(num as i32, (x as i32, y as i32));
                        map.insert((x as i32, y as i32), char::from_u32(num).unwrap());
                    }
                    None => {
                        map.insert((x as i32, y as i32), char);
                    }
                }
            }
        }

        let mut dists: HashMap<(i32, i32), i32> = HashMap::new();

        for comb in locs.keys().combinations(2) {
            let start_loc = comb[0];
            let finish_loc = comb[1];

            let (x, y) = locs[start_loc];
            let finish = locs[finish_loc];

            let shortest_path = shortest_path(&map, finish, x, y);

            dists.insert((*start_loc, *finish_loc), shortest_path);
        }

        let mut shortest = 1000000000;
        for perm in locs.keys().permutations(locs.len()) {
            if *perm[0] != 0 {
                continue;
            }
            let mut dist = 0;
            for i in 0..locs.len() - 1 {
                let start = perm[i];
                let finish = perm[i + 1];
                if dists.contains_key(&(*start, *finish)) {
                    dist += dists[&(*start, *finish)];
                } else {
                    dist += dists[&(*finish, *start)];
                }
            }
            if dist < shortest {
                shortest = dist;
            }
        }

        return shortest.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let lines = file_reader::read_lines(path);

        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let mut locs: HashMap<i32, (i32, i32)> = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char.to_digit(10) {
                    Some(num) => {
                        locs.insert(num as i32, (x as i32, y as i32));
                        map.insert((x as i32, y as i32), char::from_u32(num).unwrap());
                    }
                    None => {
                        map.insert((x as i32, y as i32), char);
                    }
                }
            }
        }

        let mut dists: HashMap<(i32, i32), i32> = HashMap::new();

        for comb in locs.keys().combinations(2) {
            let start_loc = comb[0];
            let finish_loc = comb[1];

            let (x, y) = locs[start_loc];
            let finish = locs[finish_loc];

            let shortest_path = shortest_path(&map, finish, x, y);

            dists.insert((*start_loc, *finish_loc), shortest_path);
        }

        let mut shortest = 1000000000;
        for perm in locs.keys().permutations(locs.len()) {
            if *perm[0] != 0 {
                continue;
            }
            let mut new_perm = perm.clone();
            new_perm.push(&0);
            let mut dist = 0;
            for i in 0..locs.len() {
                let start = new_perm[i];
                let finish = new_perm[i + 1];
                if dists.contains_key(&(*start, *finish)) {
                    dist += dists[&(*start, *finish)];
                } else {
                    dist += dists[&(*finish, *start)];
                }
            }
            if dist < shortest {
                shortest = dist;
            }
        }

        return shortest.to_string();
    }
}

fn shortest_path(map: &HashMap<(i32, i32), char>, finish: (i32, i32), x: i32, y: i32) -> i32 {
    let mut pq = seed_pq(&map, x, y);

    let mut dist_num;
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();
    while x != finish.0 || y != finish.1 {
        if pq.is_empty() {
            break;
        }
        let ((x, y), dist) = pq.pop().unwrap();
        dist_num = dist.0;
        points.insert((x, y), dist.0);
        for dir in [[1, 0], [-1, 0], [0, 1], [0, -1]] {
            let next_x = x + dir[0];
            let next_y = y + dir[1];
            let item = pq.get(&(next_x, next_y));
            match item {
                Some(item) => {
                    if dist_num + 1 < item.1 .0 {
                        pq.change_priority(&(next_x, next_y), Reverse(dist_num + 1));
                    }
                }
                None => {}
            }
        }
    }

    return points[&finish];
}

fn seed_pq(
    map: &HashMap<(i32, i32), char>,
    x: i32,
    y: i32,
) -> PriorityQueue<(i32, i32), Reverse<i32>> {
    let mut pq: PriorityQueue<(i32, i32), Reverse<i32>> = PriorityQueue::new();

    for (key, val) in map {
        if *val != '#' {
            if key.0 == x && key.1 == y {
                pq.push((x, y), Reverse(0));
            } else {
                pq.push((key.0, key.1), Reverse(1000000));
            }
        }
    }

    return pq;
}
