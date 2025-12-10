use anyhow::Result;
use std::collections::HashMap;
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
        for (idx, machine) in machines.iter().enumerate() {
            let mut cache: HashMap<Vec<bool>, usize> = HashMap::new();
            let mut state = vec![];
            for _ in &machine.indicators {
                state.push(false);
            }
            let mut counts: Vec<_> = vec![];
            for (button_idx, _) in machine.buttons.iter().enumerate() {
                counts.push(configure_indicators(
                    &machine,
                    state.clone(),
                    vec![],
                    button_idx,
                    &mut 10,
                    &mut cache,
                ));
            }
            let min = counts.iter().min().unwrap();
            println!("{}: {}", idx, min);
            sum += min;
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
        for (idx, machine) in machines.iter().enumerate() {
            let mut cache: HashMap<Vec<i32>, usize> = HashMap::new();
            let mut state = vec![];
            for _ in &machine.joltages {
                state.push(0);
            }
            let mut counts: Vec<_> = vec![];
            for (button_idx, _) in machine.buttons.iter().enumerate() {
                counts.push(configure_joltages(
                    &machine,
                    state.clone(),
                    vec![],
                    button_idx,
                    &mut 15,
                    &mut cache,
                ));
            }
            let min = counts.iter().min().unwrap();
            println!("{}: {}", idx, min);
            sum += min;
        }

        return Ok(sum.to_string());
    }
}

fn configure_indicators(
    machine: &Machine,
    mut state: Vec<bool>,
    mut presses: Vec<usize>,
    to_press: usize,
    shortest: &mut usize,
    cache: &mut HashMap<Vec<bool>, usize>,
) -> usize {
    if cache.contains_key(&state.clone()) && cache[&state.clone()] <= presses.len() {
        return cache[&state];
    }
    if presses.len() >= *shortest {
        cache.insert(state, presses.len());
        return presses.len();
    }
    if machine.indicators == state {
        if presses.len() < *shortest {
            *shortest = presses.len();
        }
        cache.insert(state, presses.len());
        return presses.len();
    }
    presses.push(to_press);

    press_indicators(machine, &mut state, to_press);

    let mut temp_presses: Vec<usize> = vec![];
    for (button_idx, _) in machine.buttons.iter().enumerate() {
        if button_idx == to_press {
            continue;
        }
        let count = configure_indicators(
            machine,
            state.clone(),
            presses.clone(),
            button_idx,
            shortest,
            cache,
        );
        temp_presses.push(count);
    }

    let temp_shortest = *temp_presses.iter().min().unwrap();

    if temp_shortest < *shortest {
        *shortest = temp_shortest;
    }

    cache.insert(state, temp_shortest);

    return temp_shortest;
}

fn press_indicators(machine: &Machine, state: &mut Vec<bool>, button_idx: usize) {
    for indicator in &machine.buttons[button_idx] {
        state[*indicator] = !state[*indicator];
    }
}

fn configure_joltages(
    machine: &Machine,
    mut state: Vec<i32>,
    mut presses: Vec<usize>,
    to_press: usize,
    shortest: &mut usize,
    cache: &mut HashMap<Vec<i32>, usize>,
) -> usize {
    if cache.contains_key(&state.clone()) && cache[&state.clone()] <= presses.len() {
        return cache[&state];
    }
    for (idx, joltage) in machine.joltages.iter().enumerate() {
        if state[idx] > *joltage {
            cache.insert(state, 15);
            return 15;
        }
    }
    if presses.len() >= *shortest {
        cache.insert(state, 15);
        return 15;
    }
    if machine.joltages == state {
        if presses.len() < *shortest {
            *shortest = presses.len();
        }
        cache.insert(state, presses.len());
        return presses.len();
    }
    presses.push(to_press);

    press_joltages(machine, &mut state, to_press);

    let mut temp_presses: Vec<usize> = vec![];
    for (button_idx, _) in machine.buttons.iter().enumerate() {
        let count = configure_joltages(
            machine,
            state.clone(),
            presses.clone(),
            button_idx,
            shortest,
            cache,
        );
        temp_presses.push(count);
    }

    let temp_shortest = *temp_presses.iter().min().unwrap();

    if temp_shortest < *shortest {
        *shortest = temp_shortest;
    }

    cache.insert(state, temp_shortest);

    return temp_shortest;
}

fn press_joltages(machine: &Machine, state: &mut Vec<i32>, button_idx: usize) {
    for indicator in &machine.buttons[button_idx] {
        state[*indicator] += 1;
    }
}
