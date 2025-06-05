use std::collections::{HashSet, VecDeque};

use utils::{file_reader, harness::Solve};

pub struct D19;

impl Solve for D19 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let input: i32 = inputs.first().unwrap().parse().unwrap();

        let mut participants: HashSet<i32> = HashSet::new();
        for i in 1..input + 1 {
            participants.insert(i);
        }

        while participants.len() > 1 {
            let mut participants_clone = participants.clone();
            let mut keys: Vec<i32> = participants_clone.drain().collect();
            keys.sort();
            for (i, key) in keys.iter().enumerate() {
                if keys.len() % 2 != 0 {
                    if i % 2 != 0 || i == 0 {
                        participants.remove(key);
                    }
                } else {
                    if i % 2 != 0 {
                        participants.remove(key);
                    }
                }
            }
        }

        let ans: i32 = participants.drain().collect::<Vec<i32>>()[0];

        return ans.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let input: i32 = inputs.first().unwrap().parse().unwrap();

        let mut right: VecDeque<i32> = VecDeque::new();
        let mut left: VecDeque<i32> = VecDeque::new();
        for i in 1..input + 1 {
            if i <= (input / 2) + 1 {
                right.push_back(i);
            } else {
                left.push_back(i);
            }
        }

        loop {
            if (left.len() == 0 && right.len() == 1) || (left.len() == 1 && right.len() == 0) {
                break;
            }
            if right.len() > left.len() {
                right.pop_back();

                let head = right.pop_front().unwrap();
                left.push_back(head);

                let tail = left.pop_front().unwrap();
                right.push_back(tail);
            } else {
                left.pop_front();

                let head = right.pop_front().unwrap();
                left.push_back(head);

                let tail = left.pop_front().unwrap();
                right.push_back(tail);
            }
        }

        let ans: i32 = if left.len() == 1 {
            left.pop_front().unwrap()
        } else {
            right.pop_front().unwrap()
        };

        return ans.to_string();
    }
}
