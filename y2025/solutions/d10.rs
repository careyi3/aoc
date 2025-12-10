use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use utils::{file_reader, harness::SolveResult};

pub struct D10;

struct Machine {
    indicators: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<i32>,
}

fn parse_line(line: &str) -> Machine {
    let mut indicators = Vec::new();
    let mut buttons = Vec::new();
    let mut joltages = Vec::new();

    let parts: Vec<&str> = line.split_whitespace().collect();

    for part in parts {
        if part.starts_with('[') && part.ends_with(']') {
            let content = &part[1..part.len() - 1];
            indicators = content
                .chars()
                .map(|c| if c == '.' { false } else { true })
                .collect();
        } else if part.starts_with('(') && part.ends_with(')') {
            let content = &part[1..part.len() - 1];
            let nums: Vec<usize> = content
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            buttons.push(nums);
        } else if part.starts_with('{') && part.ends_with('}') {
            let content = &part[1..part.len() - 1];
            joltages = content
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
        }
    }

    Machine {
        indicators,
        buttons,
        joltages,
    }
}

impl SolveResult for D10 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut machines: Vec<Machine> = vec![];
        for line in &input {
            let machine = parse_line(line);
            machines.push(machine);
        }

        let mut sum = 0;
        for machine in &machines {
            let mut queue: VecDeque<(Vec<bool>, usize)> = VecDeque::new();
            let mut cache: HashSet<Vec<bool>> = HashSet::new();
            queue.push_back((vec![false; machine.indicators.len()], 0));
            let depth;
            'outer: loop {
                let state = queue.pop_back().unwrap();
                cache.insert(state.0.clone());
                if state.0 == machine.indicators {
                    depth = state.1;
                    break 'outer;
                }
                for (idx, _) in machine.buttons.iter().enumerate() {
                    let new_state = press_indicator(machine, state.clone(), idx);
                    if new_state.0 == machine.indicators {
                        depth = new_state.1;
                        break 'outer;
                    }
                    if !cache.contains(&new_state.0) {
                        cache.insert(new_state.0.clone());
                        queue.push_front(new_state);
                    }
                }
            }
            sum += depth;
        }

        return Ok(sum.to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut machines: Vec<Machine> = vec![];
        for line in &input {
            let machine = parse_line(line);
            machines.push(machine);
        }

        let mut sum = 0;
        for machine in &machines {
            let mut queue: VecDeque<(Vec<i32>, usize)> = VecDeque::new();
            let mut cache: HashSet<Vec<i32>> = HashSet::new();
            queue.push_back((vec![0; machine.joltages.len()], 0));
            let depth;
            'outer: loop {
                let state = queue.pop_back().unwrap();
                cache.insert(state.0.clone());
                if state.0 == machine.joltages {
                    depth = state.1;
                    break 'outer;
                }
                for (idx, _) in machine.buttons.iter().enumerate() {
                    let new_state = press_joltage(machine, state.clone(), idx);
                    if new_state.0 == machine.joltages {
                        depth = new_state.1;
                        break 'outer;
                    }
                    if !cache.contains(&new_state.0) {
                        cache.insert(new_state.0.clone());
                        queue.push_front(new_state);
                    }
                }
            }
            sum += depth;
        }

        return Ok(sum.to_string());
    }
}

fn press_indicator(
    machine: &Machine,
    state: (Vec<bool>, usize),
    button: usize,
) -> (Vec<bool>, usize) {
    let mut new_state = state.clone();
    for indicator in &machine.buttons[button] {
        new_state.0[*indicator] = !new_state.0[*indicator];
    }
    new_state.1 += 1;
    new_state
}

fn press_joltage(machine: &Machine, state: (Vec<i32>, usize), button: usize) -> (Vec<i32>, usize) {
    let mut new_state = state.clone();
    for indicator in &machine.buttons[button] {
        new_state.0[*indicator] += 1;
    }
    new_state.1 += 1;
    new_state
}
