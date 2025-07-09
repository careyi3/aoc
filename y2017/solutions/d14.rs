use std::collections::{HashMap, HashSet};

use utils::{file_reader, harness::Solve};

pub struct D14;

impl Solve for D14 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_line(path);
        let mut ones = 0;
        for i in 0..128 {
            let output = knot_hash(format!("{}-{}", input, i));
            for c in output.chars() {
                if c == '1' {
                    ones += 1;
                }
            }
        }

        return ones.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_line(path);
        let mut map: HashMap<(i32, i32), i32> = HashMap::new();
        let mut to_visit: HashSet<(i32, i32)> = HashSet::new();
        for y in 0..128 {
            let output = knot_hash(format!("{}-{}", input, y));
            for (x, c) in output.chars().enumerate() {
                if c == '1' {
                    map.insert((x.try_into().unwrap(), y), 0);
                    to_visit.insert((x.try_into().unwrap(), y));
                }
            }
        }

        let mut groups = 0;
        while to_visit.len() > 0 {
            let (x, y) = to_visit.iter().nth(0).unwrap().clone();
            walk(&mut map, &mut to_visit, x, y);
            groups += 1;
        }

        return groups.to_string();
    }
}

fn walk(map: &mut HashMap<(i32, i32), i32>, to_visit: &mut HashSet<(i32, i32)>, x: i32, y: i32) {
    if !map.contains_key(&(x, y)) {
        return;
    }
    if map[&(x, y)] == 1 {
        return;
    }
    to_visit.remove(&(x, y));
    map.insert((x, y), 1);

    for (new_x, new_y) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        walk(map, to_visit, x + new_x, y + new_y);
    }
}

fn knot_hash(input: String) -> String {
    let chars: Vec<char> = input.chars().collect();

    let mut lengths: Vec<usize> = vec![];
    for char in chars {
        lengths.push(char as usize);
    }
    let mut suffix: Vec<usize> = vec![17, 31, 73, 47, 23];
    lengths.append(&mut suffix);
    let mut nums: Vec<usize> = (0..=255).collect();

    let mut position = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for length in &lengths {
            let target = length / 2;
            for i in 0..target {
                let nums_len = nums.len();
                let position_one = (position + i) % nums_len;
                let position_two = ((position + length - 1) - i) % nums_len;
                let one = nums[position_one];
                let two = nums[position_two];
                nums[position_two] = one;
                nums[position_one] = two;
            }
            position = (position + skip + length) % nums.len();
            skip += 1;
        }
    }

    let mut answer: Vec<u8> = vec![];
    let mut xor: u8 = 0;
    for (idx, num) in nums.iter().enumerate() {
        if idx == 0 {
            xor = *num as u8;
            continue;
        }
        if idx % 16 == 0 {
            answer.push(xor);
            xor = *num as u8;
        } else {
            xor = xor ^ *num as u8
        }
    }
    answer.push(xor);

    let mut bit_string = "".to_string();
    for num in answer {
        bit_string.push_str(&format!("{:08b}", num));
    }

    return bit_string;
}
