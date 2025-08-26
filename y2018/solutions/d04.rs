use chrono::{NaiveDateTime, Timelike};
use fancy_regex::Regex;
use itertools::Itertools;
use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D04;

impl Solve for D04 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let events: Vec<(NaiveDateTime, Vec<String>)> = inputs
            .iter()
            .map(|x| parse_event(x).unwrap())
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .collect();

        let mut guards: HashMap<String, Vec<i32>> = HashMap::new();
        let mut guard = "".to_string();
        let mut last_event: Option<NaiveDateTime> = None;
        for event in events {
            if event.1[0] == "Guard" {
                guard = event.1[1].clone();
                continue;
            }
            if event.1[0] == "falls" {
                last_event = Some(event.0);
            } else {
                let from = last_event.unwrap().minute();
                let to = event.0.minute();
                for x in from..to {
                    guards
                        .entry(guard.clone())
                        .and_modify(|v| v.push(x as i32))
                        .or_insert(vec![x as i32]);
                }
            }
        }

        let mut max: i32 = 0;
        let mut max_guard = "".to_string();
        for (guard, minutes) in &guards {
            let count = minutes.iter().count();
            if count as i32 > max {
                max = count as i32;
                max_guard = guard.clone();
            }
        }

        let counts = guards[&max_guard].iter().counts();

        let mut max = 0;
        let mut min = 0;
        for (minute, tally) in counts {
            if tally > max {
                max = tally;
                min = *minute;
            }
        }

        let id: i32 = max_guard.trim_start_matches('#').parse().unwrap();

        return (id * min).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let events: Vec<(NaiveDateTime, Vec<String>)> = inputs
            .iter()
            .map(|x| parse_event(x).unwrap())
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .collect();

        let mut guards: HashMap<String, Vec<i32>> = HashMap::new();
        let mut guard = "".to_string();
        let mut last_event: Option<NaiveDateTime> = None;
        for event in events {
            if event.1[0] == "Guard" {
                guard = event.1[1].clone();
                continue;
            }
            if event.1[0] == "falls" {
                last_event = Some(event.0);
            } else {
                let from = last_event.unwrap().minute();
                let to = event.0.minute();
                for x in from..to {
                    guards
                        .entry(guard.clone())
                        .and_modify(|v| v.push(x as i32))
                        .or_insert(vec![x as i32]);
                }
            }
        }

        let mut max: i32 = 0;
        let mut max_guard = "".to_string();
        let mut min = 0;
        for (guard, minutes) in &guards {
            let tally = minutes.iter().counts();
            for (minute, count) in tally {
                if count as i32 > max {
                    max = count as i32;
                    max_guard = guard.clone();
                    min = *minute;
                }
            }
        }

        let id: i32 = max_guard.trim_start_matches('#').parse().unwrap();

        return (id * min).to_string();
    }
}

fn parse_event(line: &str) -> Option<(NaiveDateTime, Vec<String>)> {
    let re = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\]\s+(.*)$").ok()?;
    let caps = re.captures(line).ok().flatten()?;
    let ts = caps.get(1)?.as_str();
    let msg = caps.get(2)?.as_str();

    let dt = NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M").ok()?;
    let words = msg.split_whitespace().map(|s| s.to_string()).collect();
    Some((dt, words))
}
