use std::collections::{HashMap, VecDeque};
use utils::{file_reader, harness::Solve};

pub struct D18;

impl Solve for D18 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut idx: i64 = 0;
        let mut reg: HashMap<String, i64> = HashMap::new();
        let mut iter_count = 0;
        while idx < inputs.len() as i64 && idx > -1 {
            if iter_count > 10000 {
                break;
            }
            let params: Vec<String> = inputs[idx as usize]
                .split(" ")
                .map(|x| x.to_string())
                .collect();
            idx = handle(idx, &mut reg, params);
            iter_count += 1;
        }

        return reg["snd"].to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        let mut zero = Program::new(0, input.clone());
        let mut one = Program::new(1, input.clone());
        let mut first_iter = true;
        while first_iter || zero.queue.len() > 0 || one.queue.len() > 0 {
            zero.run(&mut one.queue);
            one.run(&mut zero.queue);
            first_iter = false;
        }

        return one.send_count.to_string();
    }
}

fn handle(idx: i64, reg: &mut HashMap<String, i64>, params: Vec<String>) -> i64 {
    let mut next = idx + 1;
    match params[0].as_str() {
        "snd" => {
            let val = parse(reg, params[1].clone());
            reg.entry("snd".to_string())
                .and_modify(|x| *x = val)
                .or_insert(val);
        }
        "set" => {
            let val = parse(reg, params[2].clone());
            reg.entry(params[1].clone())
                .and_modify(|x| *x = val)
                .or_insert(val);
        }
        "add" => {
            let val = parse(reg, params[2].clone());
            reg.entry(params[1].clone())
                .and_modify(|x| *x += val)
                .or_insert(val);
        }
        "mul" => {
            let val = parse(reg, params[2].clone());
            reg.entry(params[1].clone())
                .and_modify(|x| *x *= val)
                .or_insert(0);
        }
        "mod" => {
            let val = parse(reg, params[2].clone());
            reg.entry(params[1].clone())
                .and_modify(|x| *x = *x % val)
                .or_insert(0);
        }
        "rcv" => {
            let val = parse(reg, params[1].clone());
            if val > 0 && reg.contains_key(&"snd".to_string()) {
                // println!("{}", reg[&"snd".to_string()]);
            }
        }
        "jgz" => {
            let x = parse(reg, params[1].clone());
            let y = parse(reg, params[2].clone());
            if x > 0 {
                next -= 1;
                next += y;
            }
        }
        _ => {}
    }
    return next;
}

fn parse(reg: &mut HashMap<String, i64>, param: String) -> i64 {
    match param.parse::<i64>() {
        Ok(val) => return val,
        Err(_) => {
            return *reg.entry(param).or_insert(0);
        }
    }
}

struct Program {
    idx: i64,
    pub queue: VecDeque<i64>,
    send_count: i64,
    reg: HashMap<String, i64>,
    commands: Vec<String>,
}

impl Program {
    pub fn new(id: i64, commands: Vec<String>) -> Self {
        let mut reg = HashMap::new();
        reg.insert("p".to_string(), id);
        Program {
            idx: 0,
            queue: VecDeque::new(),
            send_count: 0,
            reg: reg,
            commands,
        }
    }

    pub fn run(&mut self, target: &mut VecDeque<i64>) {
        let mut iters = 0;
        while self.idx < self.commands.len() as i64 && self.idx > -1 {
            if iters > 10000 {
                break;
            }
            let params: Vec<String> = self.commands[self.idx as usize]
                .split(" ")
                .map(|x| x.to_string())
                .collect();
            self.idx = self.handle(self.idx, params, target);
            iters += 1;
        }
    }

    fn handle(&mut self, idx: i64, params: Vec<String>, target: &mut VecDeque<i64>) -> i64 {
        let mut next = idx + 1;
        match params[0].as_str() {
            "snd" => {
                let val = parse(&mut self.reg, params[1].clone());
                target.push_back(val);
                self.send_count += 1;
            }
            "set" => {
                let val = parse(&mut self.reg, params[2].clone());
                self.reg
                    .entry(params[1].clone())
                    .and_modify(|x| *x = val)
                    .or_insert(val);
            }
            "add" => {
                let val = parse(&mut self.reg, params[2].clone());
                self.reg
                    .entry(params[1].clone())
                    .and_modify(|x| *x += val)
                    .or_insert(val);
            }
            "mul" => {
                let val = parse(&mut self.reg, params[2].clone());
                self.reg
                    .entry(params[1].clone())
                    .and_modify(|x| *x *= val)
                    .or_insert(0);
            }
            "mod" => {
                let val = parse(&mut self.reg, params[2].clone());
                self.reg
                    .entry(params[1].clone())
                    .and_modify(|x| *x = *x % val)
                    .or_insert(0);
            }
            "rcv" => {
                if self.queue.len() > 0 {
                    let val = self.queue.pop_front().unwrap();
                    self.reg
                        .entry(params[1].clone())
                        .and_modify(|x| *x = val)
                        .or_insert(val);
                } else {
                    next -= 1;
                }
            }
            "jgz" => {
                let x = parse(&mut self.reg, params[1].clone());
                let y = parse(&mut self.reg, params[2].clone());
                if x > 0 {
                    next -= 1;
                    next += y;
                }
            }
            _ => {}
        }
        return next;
    }
}
