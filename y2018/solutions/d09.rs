use std::collections::{HashMap, VecDeque};
use utils::{file_reader, harness::Solve};

pub struct D09;

impl Solve for D09 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let words: Vec<String> = input
            .first()
            .unwrap()
            .split(" ")
            .map(|x| x.to_string())
            .collect();
        let players: i64 = words.first().unwrap().parse().unwrap();
        let final_score: i64 = words[6].parse().unwrap();

        let mut scores: HashMap<i64, i64> = HashMap::new();
        let mut marbles: VecDeque<i64> = VecDeque::new();
        marbles.push_back(0);
        for i in 1..=final_score {
            if i % 23 == 0 {
                let mut score = i;
                for _ in 0..7 {
                    let val = marbles.pop_back().unwrap();
                    marbles.push_front(val);
                }
                score += marbles.pop_front().unwrap();
                scores
                    .entry(i % players)
                    .and_modify(|x| *x += score)
                    .or_insert(score);
            } else {
                for _ in 0..2 {
                    let val = marbles.pop_front().unwrap();
                    marbles.push_back(val);
                }
                marbles.push_front(i);
            }
        }

        return scores.values().max().unwrap().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let words: Vec<String> = input
            .first()
            .unwrap()
            .split(" ")
            .map(|x| x.to_string())
            .collect();
        let players: i64 = words.first().unwrap().parse().unwrap();
        let final_score: i64 = words[6].parse().unwrap();

        let mut scores: HashMap<i64, i64> = HashMap::new();
        let mut marbles: VecDeque<i64> = VecDeque::new();
        marbles.push_back(0);
        for i in 1..=final_score * 100 {
            if i % 23 == 0 {
                let mut score = i;
                for _ in 0..7 {
                    let val = marbles.pop_back().unwrap();
                    marbles.push_front(val);
                }
                score += marbles.pop_front().unwrap();
                scores
                    .entry(i % players)
                    .and_modify(|x| *x += score)
                    .or_insert(score);
            } else {
                for _ in 0..2 {
                    let val = marbles.pop_front().unwrap();
                    marbles.push_back(val);
                }
                marbles.push_front(i);
            }
        }

        return scores.values().max().unwrap().to_string();
    }
}
