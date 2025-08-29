use std::collections::VecDeque;

use utils::{file_reader, harness::Solve};

pub struct D08;

impl Solve for D08 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        let mut nums: VecDeque<i32> = input[0]
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        return walk(&mut nums).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        let mut nums: VecDeque<i32> = input[0]
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        return walk_with_indexs(&mut nums).to_string();
    }
}

fn walk(nums: &mut VecDeque<i32>) -> i32 {
    let nodes = nums.pop_front().unwrap();
    let metadata = nums.pop_front().unwrap();

    let mut sum = 0;
    for _ in 0..nodes {
        sum += walk(nums);
    }

    for _ in 0..metadata {
        sum += nums.pop_front().unwrap();
    }
    return sum;
}

fn walk_with_indexs(nums: &mut VecDeque<i32>) -> i32 {
    let nodes = nums.pop_front().unwrap();
    let metadata = nums.pop_front().unwrap();

    let mut sum = 0;
    if nodes > 0 {
        let mut vals: Vec<i32> = vec![];
        for _ in 0..nodes {
            vals.push(walk_with_indexs(nums));
        }
        let mut ids: Vec<i32> = vec![];
        for _ in 0..metadata {
            ids.push(nums.pop_front().unwrap());
        }

        for id in ids {
            if id == 0 || id - 1 > (vals.len() - 1) as i32 {
                continue;
            }
            sum += vals[(id - 1) as usize];
        }
    } else {
        for _ in 0..metadata {
            sum += nums.pop_front().unwrap();
        }
    }
    return sum;
}
