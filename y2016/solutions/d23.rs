use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D23;

impl Solve for D23 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let (mut registers, commands) = setup(7);

        let mut instructions: Vec<Vec<String>> = vec![];
        for input in inputs {
            let mut temp: Vec<String> = input.split(" ").map(|f| f.to_string()).collect();
            if temp.len() == 2 {
                temp.push("".to_string());
            }
            instructions.push(temp);
        }

        let mut i: i32 = 0;
        while i < instructions.len().try_into().unwrap() {
            let idx: usize = i.try_into().unwrap();
            let instruction: Vec<String> = instructions[idx].clone();

            let command = commands[&instruction[0]];
            i = command(
                i,
                &mut instructions,
                &mut registers,
                instruction[1].clone(),
                instruction[2].clone(),
            );
        }

        return registers[&"a".to_string()].to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let (mut registers, commands) = setup(6);

        let mut instructions: Vec<Vec<String>> = vec![];
        for input in inputs {
            let mut temp: Vec<String> = input.split(" ").map(|f| f.to_string()).collect();
            if temp.len() == 2 {
                temp.push("".to_string());
            }
            instructions.push(temp);
        }

        let mut i: i32 = 0;
        while i < instructions.len().try_into().unwrap() {
            let idx: usize = i.try_into().unwrap();
            let instruction: Vec<String> = instructions[idx].clone();

            let command = commands[&instruction[0]];
            i = command(
                i,
                &mut instructions,
                &mut registers,
                instruction[1].clone(),
                instruction[2].clone(),
            );
        }

        let fact_6 = factorial(6);
        let fact_12 = factorial(12);
        let offset = registers[&"a".to_string()] - fact_6;

        return (fact_12 + offset).to_string();
    }
}

fn factorial(num: i32) -> i32 {
    let mut val = 1;
    for i in (1..num + 1).rev() {
        val *= i;
    }
    return val;
}

fn setup(
    a: i32,
) -> (
    HashMap<String, i32>,
    HashMap<
        String,
        fn(i32, &mut Vec<Vec<String>>, &mut HashMap<String, i32>, String, String) -> i32,
    >,
) {
    let registers: HashMap<String, i32> = HashMap::from([
        ("a".to_string(), a),
        ("b".to_string(), 0),
        ("c".to_string(), 0),
        ("d".to_string(), 0),
    ]);

    let cpy = |id: i32,
               _: &mut Vec<Vec<String>>,
               reg: &mut HashMap<String, i32>,
               x: String,
               y: String|
     -> i32 {
        let value = match x.parse() {
            Ok(a) => a,
            Err(_) => reg[&x],
        };
        if !reg.contains_key(&y) {
            return id + 1;
        }
        reg.entry(y).and_modify(|r| *r = value);
        return id + 1;
    };

    let inc = |id: i32,
               _: &mut Vec<Vec<String>>,
               reg: &mut HashMap<String, i32>,
               x: String,
               _: String|
     -> i32 {
        if !reg.contains_key(&x) {
            return id + 1;
        }
        reg.entry(x).and_modify(|r| *r += 1);
        return id + 1;
    };

    let dec = |id: i32,
               _: &mut Vec<Vec<String>>,
               reg: &mut HashMap<String, i32>,
               x: String,
               _: String|
     -> i32 {
        if !reg.contains_key(&x) {
            return id + 1;
        }
        reg.entry(x).and_modify(|r| *r -= 1);
        return id + 1;
    };

    let jnz = |id: i32,
               _: &mut Vec<Vec<String>>,
               reg: &mut HashMap<String, i32>,
               x: String,
               y: String|
     -> i32 {
        let value = match x.parse() {
            Ok(a) => a,
            Err(_) => reg[&x],
        };
        let offset = match y.parse() {
            Ok(a) => a,
            Err(_) => reg[&y],
        };
        if value != 0 {
            return id + offset;
        }
        return id + 1;
    };

    let tgl = |id: i32,
               commands: &mut Vec<Vec<String>>,
               reg: &mut HashMap<String, i32>,
               x: String,
               _: String|
     -> i32 {
        let value = match x.parse() {
            Ok(a) => a,
            Err(_) => reg[&x],
        };
        let idx: usize = (id + value) as usize;
        if idx >= commands.len() {
            return id + 1;
        }
        let ins = &commands[idx];
        let cmd = &ins[0];
        let mut new_ins: Vec<String> = vec![];
        match cmd.as_str() {
            "tgl" => {
                new_ins = vec!["inc".to_string(), ins[1].clone(), ins[2].clone()];
            }
            "inc" => {
                new_ins = vec!["dec".to_string(), ins[1].clone(), ins[2].clone()];
            }
            "dec" => {
                new_ins = vec!["inc".to_string(), ins[1].clone(), ins[2].clone()];
            }
            "jnz" => {
                new_ins = vec!["cpy".to_string(), ins[1].clone(), ins[2].clone()];
            }
            "cpy" => {
                new_ins = vec!["jnz".to_string(), ins[1].clone(), ins[2].clone()];
            }
            _ => {}
        }
        commands[idx] = new_ins;
        return id + 1;
    };

    let commands: HashMap<
        String,
        fn(i32, &mut Vec<Vec<String>>, &mut HashMap<String, i32>, String, String) -> i32,
    > = HashMap::from([
        (
            "cpy".to_string(),
            cpy as fn(i32, &mut Vec<Vec<String>>, &mut HashMap<String, i32>, String, String) -> i32,
        ),
        (
            "inc".to_string(),
            inc as fn(i32, &mut Vec<Vec<String>>, &mut HashMap<String, i32>, String, String) -> i32,
        ),
        (
            "dec".to_string(),
            dec as fn(i32, &mut Vec<Vec<String>>, &mut HashMap<String, i32>, String, String) -> i32,
        ),
        (
            "jnz".to_string(),
            jnz as fn(i32, &mut Vec<Vec<String>>, &mut HashMap<String, i32>, String, String) -> i32,
        ),
        (
            "tgl".to_string(),
            tgl as fn(i32, &mut Vec<Vec<String>>, &mut HashMap<String, i32>, String, String) -> i32,
        ),
    ]);
    return (registers, commands);
}
