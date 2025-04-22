use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D06;

impl Solve for D06 {
    fn part1(_input: String, path: &String) -> String {
        let instructions = file_reader::read_lines(path);

        let mut state: HashMap<(i32, i32), i32> = HashMap::new();
        for instruction in instructions {
            let parts: Vec<&str> = instruction.split(' ').collect();
            if parts.len() == 4 {
                update_state_p1(&mut state, 0, parts[1], parts[3]);
            } else {
                if parts[1] == "on" {
                    update_state_p1(&mut state, 1, parts[2], parts[4]);
                } else {
                    update_state_p1(&mut state, 2, parts[2], parts[4]);
                }
            }
        }

        let count: i32 = state.values().sum();

        return count.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let instructions = file_reader::read_lines(path);

        let mut state: HashMap<(i32, i32), i32> = HashMap::new();
        for instruction in instructions {
            let parts: Vec<&str> = instruction.split(' ').collect();
            if parts.len() == 4 {
                update_state_p2(&mut state, 0, parts[1], parts[3]);
            } else {
                if parts[1] == "on" {
                    update_state_p2(&mut state, 1, parts[2], parts[4]);
                } else {
                    update_state_p2(&mut state, 2, parts[2], parts[4]);
                }
            }
        }

        let sum: i32 = state.values().sum();

        return sum.to_string();
    }
}

fn update_state_p1(state: &mut HashMap<(i32, i32), i32>, op: i32, from: &str, to: &str) {
    let from_coord: Vec<i32> = from.split(',').map(|x| x.parse().unwrap()).collect();
    let to_coord: Vec<i32> = to.split(',').map(|x| x.parse().unwrap()).collect();
    let xs = [from_coord[0], to_coord[0]];
    let xfrom = xs.iter().min().unwrap();
    let xto = xs.iter().max().unwrap();
    let ys = [from_coord[1], to_coord[1]];
    let yfrom = ys.iter().min().unwrap();
    let yto = ys.iter().max().unwrap();
    for x in *xfrom..*xto + 1 {
        for y in *yfrom..*yto + 1 {
            if op == 0 {
                state
                    .entry((x, y))
                    .and_modify(|val| if *val == 1 { *val = 0 } else { *val = 1 })
                    .or_insert(1);
            }
            if op == 1 {
                state.entry((x, y)).and_modify(|val| *val = 1).or_insert(1);
            }
            if op == 2 {
                state.entry((x, y)).and_modify(|val| *val = 0).or_insert(0);
            }
        }
    }
}

fn update_state_p2(state: &mut HashMap<(i32, i32), i32>, op: i32, from: &str, to: &str) {
    let from_coord: Vec<i32> = from.split(',').map(|x| x.parse().unwrap()).collect();
    let to_coord: Vec<i32> = to.split(',').map(|x| x.parse().unwrap()).collect();
    let xs = [from_coord[0], to_coord[0]];
    let xfrom = xs.iter().min().unwrap();
    let xto = xs.iter().max().unwrap();
    let ys = [from_coord[1], to_coord[1]];
    let yfrom = ys.iter().min().unwrap();
    let yto = ys.iter().max().unwrap();
    for x in *xfrom..*xto + 1 {
        for y in *yfrom..*yto + 1 {
            if op == 0 {
                state.entry((x, y)).and_modify(|val| *val += 2).or_insert(2);
            }
            if op == 1 {
                state.entry((x, y)).and_modify(|val| *val += 1).or_insert(1);
            }
            if op == 2 {
                state
                    .entry((x, y))
                    .and_modify(|val| if *val == 0 { *val = 0 } else { *val -= 1 })
                    .or_insert(0);
            }
        }
    }
}
