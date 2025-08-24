use pprint::{Doc, Printer, PRINTER};
use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D23;

impl Solve for D23 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut idx: i64 = 0;
        let mut reg: HashMap<String, i64> = HashMap::new();
        while idx < inputs.len() as i64 && idx > -1 {
            let params: Vec<String> = inputs[idx as usize]
                .split(" ")
                .map(|x| x.to_string())
                .collect();
            idx = handle(&mut reg, idx, params);
        }
        return reg["mul"].to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut idx: i64 = 0;
        let mut reg: HashMap<String, i64> = HashMap::new();
        reg.insert("a".to_string(), 1);
        while idx < inputs.len() as i64 && idx > -1 {
            let params: Vec<String> = inputs[idx as usize]
                .split(" ")
                .map(|x| x.to_string())
                .collect();
            idx = handle(&mut reg, idx, params);
            println!("{}", idx);
            let doc = Doc::from(&reg);
            println!("{}", PRINTER.pprint(doc));
        }
        return reg["h"].to_string();
    }
}

fn handle(reg: &mut HashMap<String, i64>, id: i64, params: Vec<String>) -> i64 {
    let mut next_id = id + 1;
    match params[0].as_str() {
        "set" => {
            let val = parse(reg, params[2].clone());
            reg.entry(params[1].clone())
                .and_modify(|x| *x = val)
                .or_insert(val);
        }
        "sub" => {
            let val = parse(reg, params[2].clone());
            reg.entry(params[1].clone())
                .and_modify(|x| *x -= val)
                .or_insert(0 - val);
        }
        "mul" => {
            let val = parse(reg, params[2].clone());
            reg.entry(params[1].clone())
                .and_modify(|x| *x *= val)
                .or_insert(0);
            reg.entry("mul".to_string())
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        "jnz" => {
            let x = parse(reg, params[1].clone());
            let y = parse(reg, params[2].clone());
            if x != 0 {
                next_id -= 1;
                next_id += y;
            }
        }
        _ => {}
    }
    return next_id;
}

fn parse(reg: &mut HashMap<String, i64>, param: String) -> i64 {
    match param.parse::<i64>() {
        Ok(val) => return val,
        Err(_) => {
            return *reg.entry(param).or_insert(0);
        }
    }
}
