use std::{collections::HashMap, vec};
use utils::{file_reader, harness::Solve};

pub struct D23;

impl Solve for D23 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut program: Vec<(String, String, i32)> = vec![];
        for input in inputs {
            let chars: Vec<String> = input.split(' ').map(|x| x.to_string()).collect();
            let mut instruction: (String, String, i32) = ("".to_string(), "".to_string(), 0);
            instruction.0 = chars[0].clone();
            instruction.1 = chars[1].clone();
            if chars.len() == 3 {
                instruction.2 = chars[2].clone().parse().unwrap();
            }
            program.push(instruction);
        }

        let mut registers: HashMap<String, i32> =
            HashMap::from([("a".to_string(), 0), ("b".to_string(), 0)]);
        let commands = setup();

        let mut idx: usize = 0;
        while idx < program.len() {
            let instruction = &program[idx];
            let func = commands[&instruction.0];
            idx = func(
                idx,
                &mut registers.get_mut(&instruction.1).unwrap(),
                instruction.2,
            );
        }

        return registers["b"].to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut program: Vec<(String, String, i32)> = vec![];
        for input in inputs {
            let chars: Vec<String> = input.split(' ').map(|x| x.to_string()).collect();
            let mut instruction: (String, String, i32) = ("".to_string(), "".to_string(), 0);
            instruction.0 = chars[0].clone();
            instruction.1 = chars[1].clone();
            if chars.len() == 3 {
                instruction.2 = chars[2].clone().parse().unwrap();
            }
            program.push(instruction);
        }

        let mut registers: HashMap<String, i32> =
            HashMap::from([("a".to_string(), 1), ("b".to_string(), 0)]);
        let commands = setup();

        let mut idx: usize = 0;
        while idx < program.len() {
            let instruction = &program[idx];
            let func = commands[&instruction.0];
            idx = func(
                idx,
                &mut registers.get_mut(&instruction.1).unwrap(),
                instruction.2,
            );
        }

        return registers["b"].to_string();
    }
}

fn setup() -> HashMap<String, for<'a> fn(usize, &'a mut i32, i32) -> usize> {
    let hlf = |idx: usize, reg: &mut i32, _val: i32| -> usize {
        *reg = *reg / 2;
        return idx + 1;
    };
    let tpl = |idx: usize, reg: &mut i32, _val: i32| -> usize {
        *reg = *reg * 3;
        return idx + 1;
    };
    let inc = |idx: usize, reg: &mut i32, _val: i32| -> usize {
        *reg += 1;
        return idx + 1;
    };
    let jmp = |idx: usize, _reg: &mut i32, val: i32| -> usize {
        return add(idx, val);
    };
    let jie = |idx: usize, reg: &mut i32, val: i32| -> usize {
        if *reg % 2 == 0 {
            return add(idx, val);
        }
        return idx + 1;
    };
    let jio = |idx: usize, reg: &mut i32, val: i32| -> usize {
        if *reg == 1 {
            return add(idx, val);
        }
        return idx + 1;
    };

    let commands = HashMap::from([
        ("hlf".to_string(), hlf as fn(usize, &mut i32, i32) -> usize),
        ("tpl".to_string(), tpl as fn(usize, &mut i32, i32) -> usize),
        ("inc".to_string(), inc as fn(usize, &mut i32, i32) -> usize),
        ("jmp".to_string(), jmp as fn(usize, &mut i32, i32) -> usize),
        ("jie".to_string(), jie as fn(usize, &mut i32, i32) -> usize),
        ("jio".to_string(), jio as fn(usize, &mut i32, i32) -> usize),
    ]);
    return commands;
}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}
