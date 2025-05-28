use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D10;

impl Solve for D10 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut bots: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut outputs: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut rules: Vec<(i32, String, i32, String, i32)> = vec![];

        for input in inputs {
            let words: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();
            if words[0] == "value".to_string() {
                let val: i32 = words[1].parse().unwrap();
                let bot: i32 = words[5].parse().unwrap();
                bots.entry(bot)
                    .and_modify(|x| x.push(val))
                    .or_insert(vec![val]);
            } else {
                let bot: i32 = words[1].parse().unwrap();

                let low_target = words[5].clone();
                let low_target_id: i32 = words[6].parse().unwrap();
                let high_target = words[10].clone();
                let high_target_id: i32 = words[11].parse().unwrap();
                rules.push((bot, low_target, low_target_id, high_target, high_target_id));
            }
        }

        let low_test = 17;
        let high_test = 61;
        let mut answer = 0;

        while rules.len() > 0 {
            let mut new_rules = vec![];
            for (bot, low_target, low_target_id, high_target, high_target_id) in rules {
                if !bots.contains_key(&bot) {
                    bots.insert(bot, vec![]);
                }
                let mut values = bots[&bot].clone();
                if values.len() == 2 {
                    values.sort();
                    let low = values[0];
                    let high = values[1];

                    if low == low_test && high == high_test {
                        answer = bot;
                    }

                    if *low_target == "bot".to_string() {
                        bots.entry(low_target_id)
                            .and_modify(|s| s.push(low))
                            .or_insert(vec![low]);
                    } else {
                        outputs
                            .entry(low_target_id)
                            .and_modify(|s| s.push(low))
                            .or_insert(vec![low]);
                    }

                    if *high_target == "bot".to_string() {
                        bots.entry(high_target_id)
                            .and_modify(|s| s.push(high))
                            .or_insert(vec![high]);
                    } else {
                        outputs
                            .entry(high_target_id)
                            .and_modify(|s| s.push(high))
                            .or_insert(vec![high]);
                    }

                    bots.insert(bot, vec![]);
                } else {
                    new_rules.push((bot, low_target, low_target_id, high_target, high_target_id));
                }
            }

            rules = new_rules;
        }

        return answer.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut bots: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut outputs: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut rules: Vec<(i32, String, i32, String, i32)> = vec![];

        for input in inputs {
            let words: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();
            if words[0] == "value".to_string() {
                let val: i32 = words[1].parse().unwrap();
                let bot: i32 = words[5].parse().unwrap();
                bots.entry(bot)
                    .and_modify(|x| x.push(val))
                    .or_insert(vec![val]);
            } else {
                let bot: i32 = words[1].parse().unwrap();

                let low_target = words[5].clone();
                let low_target_id: i32 = words[6].parse().unwrap();
                let high_target = words[10].clone();
                let high_target_id: i32 = words[11].parse().unwrap();
                rules.push((bot, low_target, low_target_id, high_target, high_target_id));
            }
        }

        while rules.len() > 0 {
            let mut new_rules = vec![];
            for (bot, low_target, low_target_id, high_target, high_target_id) in rules {
                if !bots.contains_key(&bot) {
                    bots.insert(bot, vec![]);
                }
                let mut values = bots[&bot].clone();
                if values.len() == 2 {
                    values.sort();
                    let low = values[0];
                    let high = values[1];

                    if *low_target == "bot".to_string() {
                        bots.entry(low_target_id)
                            .and_modify(|s| s.push(low))
                            .or_insert(vec![low]);
                    } else {
                        outputs
                            .entry(low_target_id)
                            .and_modify(|s| s.push(low))
                            .or_insert(vec![low]);
                    }

                    if *high_target == "bot".to_string() {
                        bots.entry(high_target_id)
                            .and_modify(|s| s.push(high))
                            .or_insert(vec![high]);
                    } else {
                        outputs
                            .entry(high_target_id)
                            .and_modify(|s| s.push(high))
                            .or_insert(vec![high]);
                    }

                    bots.insert(bot, vec![]);
                } else {
                    new_rules.push((bot, low_target, low_target_id, high_target, high_target_id));
                }
            }

            rules = new_rules;
        }

        return (outputs[&0][0] * outputs[&1][0] * outputs[&2][0]).to_string();
    }
}
