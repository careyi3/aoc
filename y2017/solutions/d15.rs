use std::collections::VecDeque;

use utils::{file_reader, harness::Solve};

pub struct D15;

impl Solve for D15 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let mut seeds: Vec<u64> = vec![];
        for line in input {
            let tokens: Vec<String> = line.split(" ").map(|c| c.to_string()).collect();
            seeds.push(tokens.last().unwrap().parse().unwrap())
        }

        let mut gen_a = seeds[0];
        let mut gen_b = seeds[1];
        let mut i = 0;
        let mut matches: Vec<u64> = vec![];
        while i < 40000000 {
            gen_a = (gen_a * 16807) % 2147483647;
            gen_b = (gen_b * 48271) % 2147483647;
            if gen_a as u16 == gen_b as u16 {
                matches.push(i);
            }
            i += 1;
        }

        return matches.len().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let mut seeds: Vec<u64> = vec![];
        for line in input {
            let tokens: Vec<String> = line.split(" ").map(|c| c.to_string()).collect();
            seeds.push(tokens.last().unwrap().parse().unwrap())
        }

        let mut gen_a = seeds[0];
        let mut gen_b = seeds[1];
        let mut a_s: VecDeque<u16> = VecDeque::new();
        let mut b_s: VecDeque<u16> = VecDeque::new();
        let mut matched = 0;
        let mut checked = 0;
        while checked < 5000000 {
            gen_a = (gen_a * 16807) % 2147483647;
            gen_b = (gen_b * 48271) % 2147483647;
            if gen_a % 4 == 0 {
                a_s.push_front(gen_a as u16);
            }
            if gen_b % 8 == 0 {
                b_s.push_front(gen_b as u16);
            }
            if a_s.len() > 0 && b_s.len() > 0 {
                let a = a_s.pop_back().unwrap();
                let b = b_s.pop_back().unwrap();
                if a == b {
                    matched += 1;
                }
                checked += 1;
            }
        }

        return matched.to_string();
    }
}
