use itertools::Itertools;
use std::collections::VecDeque;

use utils::{file_reader, harness::Solve};

pub struct D21;

impl Solve for D21 {
    fn part1(_input: String, path: &String) -> String {
        let inputs: Vec<String> = file_reader::read_lines(path);
        let chars: VecDeque<char> = VecDeque::from(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);

        return solve(inputs, chars);
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs: Vec<String> = file_reader::read_lines(path);
        let chars: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        let mut ans = "".to_string();
        let test = "fbgdceah".to_string();
        for per in chars.iter().permutations(8) {
            let chars = VecDeque::from(per.iter().map(|c| **c).collect::<Vec<char>>());
            let to_test = solve(inputs.clone(), chars);
            if to_test == test {
                ans = per.iter().map(|x| **x).collect();
                break;
            }
        }

        return ans;
    }
}

fn solve(inputs: Vec<String>, mut chars: VecDeque<char>) -> String {
    for input in inputs {
        let params: Vec<String> = input.split(' ').map(|x| x.to_string()).collect();
        if params[0] == "swap".to_string() {
            if params[1] == "position".to_string() {
                let x: usize = params[2].parse().unwrap();
                let y: usize = params[5].parse().unwrap();
                swap_position(&mut chars, x, y);
            } else {
                let x = params[2].chars().next().unwrap();
                let y = params[5].chars().next().unwrap();
                swap_letters(&mut chars, x, y);
            }
        } else if params[0] == "rotate".to_string() {
            if params[1] == "left".to_string() {
                let n: usize = params[2].parse().unwrap();
                rotate_left(&mut chars, n);
            } else if params[1] == "right".to_string() {
                let n: usize = params[2].parse().unwrap();
                rotate_right(&mut chars, n);
            } else {
                let char = params[6].chars().next().unwrap();
                rotate_from_position(&mut chars, char);
            }
        } else if params[0] == "reverse".to_string() {
            let x: usize = params[2].parse().unwrap();
            let y: usize = params[4].parse().unwrap();
            reverse_positions(&mut chars, x, y);
        } else if params[0] == "move".to_string() {
            let x: usize = params[2].parse().unwrap();
            let y: usize = params[5].parse().unwrap();
            move_position(&mut chars, x, y);
        }
    }
    return chars.iter().collect();
}

fn swap_position(chars: &mut VecDeque<char>, x: usize, y: usize) {
    let temp_x = *chars.get(x).unwrap();
    let temp_y = *chars.get(y).unwrap();

    chars[x] = temp_y;
    chars[y] = temp_x;
}

fn swap_letters(chars: &mut VecDeque<char>, x: char, y: char) {
    for i in 0..chars.len() {
        if *chars.get(i).unwrap() == x {
            chars[i] = y;
            continue;
        }
        if *chars.get(i).unwrap() == y {
            chars[i] = x;
            continue;
        }
    }
}

fn rotate_right(chars: &mut VecDeque<char>, n: usize) {
    for _ in 0..n {
        let temp = chars.pop_back().unwrap();
        chars.push_front(temp);
    }
}

fn rotate_left(chars: &mut VecDeque<char>, n: usize) {
    for _ in 0..n {
        let temp = chars.pop_front().unwrap();
        chars.push_back(temp);
    }
}

fn rotate_from_position(chars: &mut VecDeque<char>, char: char) {
    let mut idx = 0;
    for i in 0..chars.len() {
        if *chars.get(i).unwrap() == char {
            idx = i;
            break;
        }
    }

    let mut n = 1 + idx;
    if idx >= 4 {
        n += 1;
    }

    rotate_right(chars, n);
}

fn reverse_positions(chars: &mut VecDeque<char>, x: usize, y: usize) {
    let mut sub: Vec<char> = vec![];
    for i in x..=y {
        sub.push(*chars.get(i).unwrap());
    }
    sub.reverse();

    for (i, idx) in (x..=y).enumerate() {
        chars[idx] = sub[i];
    }
}

fn move_position(chars: &mut VecDeque<char>, x: usize, y: usize) {
    let val = chars.remove(x).unwrap();
    if y < chars.len() {
        chars.insert(y, val);
    } else {
        chars.push_back(val);
    }
}
