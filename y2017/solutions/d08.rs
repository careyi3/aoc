use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D08;

impl Solve for D08 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut registers: HashMap<String, i32> = HashMap::new();
        for input in inputs {
            let tokens: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
            let op_reg = &tokens[0];
            let op = &tokens[1];
            let op_val: i32 = tokens[2].parse().unwrap();

            let con_reg = &tokens[4];
            let con = &tokens[5];
            let con_val: i32 = tokens[6].parse().unwrap();

            if condition(&mut registers, con_reg.clone(), con.clone(), con_val) {
                operation(&mut registers, op_reg.clone(), op.clone(), op_val);
            }
        }

        return registers.values().max().unwrap().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut registers: HashMap<String, i32> = HashMap::new();
        let mut highest: i32 = 0;
        for input in inputs {
            let tokens: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
            let op_reg = &tokens[0];
            let op = &tokens[1];
            let op_val: i32 = tokens[2].parse().unwrap();

            let con_reg = &tokens[4];
            let con = &tokens[5];
            let con_val: i32 = tokens[6].parse().unwrap();

            if condition(&mut registers, con_reg.clone(), con.clone(), con_val) {
                operation(&mut registers, op_reg.clone(), op.clone(), op_val);
            }
            let current_highest = registers.values().max().unwrap();
            if *current_highest > highest {
                highest = *current_highest;
            }
        }

        return highest.to_string();
    }
}

fn condition(registers: &mut HashMap<String, i32>, reg: String, con: String, val: i32) -> bool {
    let test_val;
    if registers.contains_key(&reg) {
        test_val = registers[&reg];
    } else {
        registers.insert(reg, 0);
        test_val = 0;
    }

    match con.as_str() {
        ">" => {
            if test_val > val {
                return true;
            }
        }
        "<" => {
            if test_val < val {
                return true;
            }
        }
        "<=" => {
            if test_val <= val {
                return true;
            }
        }
        ">=" => {
            if test_val >= val {
                return true;
            }
        }
        "==" => {
            if test_val == val {
                return true;
            }
        }
        "!=" => {
            if test_val != val {
                return true;
            }
        }
        _ => return false,
    };
    return false;
}

fn operation(registers: &mut HashMap<String, i32>, reg: String, op: String, val: i32) {
    if op == "inc".to_string() {
        registers
            .entry(reg)
            .and_modify(|x| *x += val)
            .or_insert(val);
    } else {
        registers
            .entry(reg)
            .and_modify(|x| *x -= val)
            .or_insert(-val);
    }
}
