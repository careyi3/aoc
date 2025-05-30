use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D12;

impl Solve for D12 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let (mut registers, commands) = setup(0);

        let mut instructions: Vec<Vec<String>> = vec![];
        for input in inputs {
            instructions.push(input.split(" ").map(|f| f.to_string()).collect());
        }

        let mut i: i32 = 0;
        while i < instructions.len().try_into().unwrap() {
            let idx: usize = i.try_into().unwrap();
            let mut instruction: Vec<String> = instructions[idx].clone();
            if instruction.len() == 2 {
                instruction.push("".to_string());
            }

            let command = commands[&instruction[0]];
            i = command(
                i,
                &mut registers,
                instruction[1].clone(),
                instruction[2].clone(),
            );
        }

        return registers[&"a".to_string()].to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let (mut registers, commands) = setup(1);

        let mut instructions: Vec<Vec<String>> = vec![];
        for input in inputs {
            instructions.push(input.split(" ").map(|f| f.to_string()).collect());
        }

        let mut i: i32 = 0;
        while i < instructions.len().try_into().unwrap() {
            let idx: usize = i.try_into().unwrap();
            let mut instruction: Vec<String> = instructions[idx].clone();
            if instruction.len() == 2 {
                instruction.push("".to_string());
            }

            let command = commands[&instruction[0]];
            i = command(
                i,
                &mut registers,
                instruction[1].clone(),
                instruction[2].clone(),
            );
        }

        return registers[&"a".to_string()].to_string();
    }
}

fn setup(
    c: i32,
) -> (
    HashMap<String, i32>,
    HashMap<String, fn(i32, &mut HashMap<String, i32>, String, String) -> i32>,
) {
    let registers: HashMap<String, i32> = HashMap::from([
        ("a".to_string(), 0),
        ("b".to_string(), 0),
        ("c".to_string(), c),
        ("d".to_string(), 0),
    ]);

    let cpy = |id: i32, reg: &mut HashMap<String, i32>, x: String, y: String| -> i32 {
        let value = match x.parse() {
            Ok(a) => a,
            Err(_) => reg[&x],
        };
        reg.entry(y).and_modify(|r| *r = value);
        return id + 1;
    };

    let inc = |id: i32, reg: &mut HashMap<String, i32>, x: String, _: String| -> i32 {
        reg.entry(x).and_modify(|r| *r += 1);
        return id + 1;
    };

    let dec = |id: i32, reg: &mut HashMap<String, i32>, x: String, _: String| -> i32 {
        reg.entry(x).and_modify(|r| *r -= 1);
        return id + 1;
    };

    let jnz = |id: i32, reg: &mut HashMap<String, i32>, x: String, y: String| -> i32 {
        let value = match x.parse() {
            Ok(a) => a,
            Err(_) => reg[&x],
        };
        let offset: i32 = y.parse().unwrap();
        if value != 0 {
            return id + offset;
        }
        return id + 1;
    };

    let commands: HashMap<String, fn(i32, &mut HashMap<String, i32>, String, String) -> i32> =
        HashMap::from([
            (
                "cpy".to_string(),
                cpy as fn(i32, &mut HashMap<String, i32>, String, String) -> i32,
            ),
            (
                "inc".to_string(),
                inc as fn(i32, &mut HashMap<String, i32>, String, String) -> i32,
            ),
            (
                "dec".to_string(),
                dec as fn(i32, &mut HashMap<String, i32>, String, String) -> i32,
            ),
            (
                "jnz".to_string(),
                jnz as fn(i32, &mut HashMap<String, i32>, String, String) -> i32,
            ),
        ]);
    return (registers, commands);
}
