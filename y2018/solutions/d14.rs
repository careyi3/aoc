use std::collections::VecDeque;

use itertools::Itertools;

use utils::{file_reader, harness::Solve};

pub struct D14;

impl Solve for D14 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let num: usize = input.first().unwrap().parse().unwrap();

        let mut scores: Vec<i32> = Vec::new();
        scores.push(3);
        scores.push(7);

        let mut elf1 = 0;
        let mut elf2 = 1;

        while scores.len() < num + 10 {
            let score1 = scores[elf1];
            let score2 = scores[elf2];
            let new_score = score1 + score2;
            for c in new_score.to_string().chars() {
                let new_num = c.to_digit(10).unwrap() as i32;
                scores.push(new_num);
            }
            elf1 = (elf1 + (scores[elf1] + 1) as usize) % scores.len();
            elf2 = (elf2 + (scores[elf2] + 1) as usize) % scores.len();
        }

        let digits = &scores[num..num + 10];

        return digits.iter().join("");
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let num: usize = input.first().unwrap().parse().unwrap();

        let mut scores: Vec<i32> = Vec::new();
        scores.push(3);
        scores.push(7);

        let pattern: Vec<i32> = num
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();

        let mut window: VecDeque<i32> = VecDeque::new();
        window.push_back(3);
        window.push_back(7);

        let mut elf1 = 0;
        let mut elf2 = 1;
        'outer: loop {
            let score1 = scores[elf1];
            let score2 = scores[elf2];
            let new_score = score1 + score2;
            for c in new_score.to_string().chars() {
                let new_num = c.to_digit(10).unwrap() as i32;
                scores.push(new_num);
                if window.len() == pattern.len() {
                    window.pop_front();
                    window.push_back(new_num);
                } else {
                    window.push_back(new_num);
                }
                if window == pattern {
                    break 'outer;
                }
            }
            elf1 = (elf1 + (scores[elf1] + 1) as usize) % scores.len();
            elf2 = (elf2 + (scores[elf2] + 1) as usize) % scores.len();
        }

        return (scores.len() - pattern.len()).to_string();
    }
}
