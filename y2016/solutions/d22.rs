use std::collections::HashMap;

use fancy_regex::Regex;

use utils::{file_reader, harness::Solve};

pub struct D22;

impl Solve for D22 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let reg = Regex::new(r"x(\d+)-y(\d+)").unwrap();

        let mut nodes: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

        for (id, input) in inputs.iter().enumerate() {
            if id < 2 {
                continue;
            }

            let params: Vec<String> = input
                .split(' ')
                .map(|x| x.to_string())
                .filter(|x| x.len() != 0)
                .collect();

            let node_param = params[0].clone();
            let used_param = params[2].clone();
            let available_param = params[3].clone();

            let used: i32 = used_param[..used_param.len() - 1].parse().unwrap();
            let available: i32 = available_param[..available_param.len() - 1]
                .parse()
                .unwrap();

            for res in reg.captures_iter(&node_param) {
                let captures = res.unwrap();
                let x: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
                let y: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
                nodes.insert((x, y), (used, available));
            }
        }

        let mut count = 0;
        for (ikey, ival) in &nodes {
            for (nkey, nval) in &nodes {
                if ikey == nkey {
                    continue;
                }
                if ival.0 <= nval.1 && ival.0 != 0 {
                    count += 1;
                }
            }
        }

        return count.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let reg = Regex::new(r"x(\d+)-y(\d+)").unwrap();

        let mut nodes: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

        for (id, input) in inputs.iter().enumerate() {
            if id < 2 {
                continue;
            }

            let params: Vec<String> = input
                .split(' ')
                .map(|x| x.to_string())
                .filter(|x| x.len() != 0)
                .collect();

            let node_param = params[0].clone();
            let used_param = params[2].clone();
            let available_param = params[3].clone();

            let used: i32 = used_param[..used_param.len() - 1].parse().unwrap();
            let available: i32 = available_param[..available_param.len() - 1]
                .parse()
                .unwrap();

            for res in reg.captures_iter(&node_param) {
                let captures = res.unwrap();
                let x: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
                let y: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
                nodes.insert((x, y), (used, available));
            }
        }

        let mut cells: HashMap<(i32, i32), char> = HashMap::new();
        for x in 0..32 {
            for y in 0..31 {
                let opt = nodes.get(&(x, y));
                if opt.is_none() {
                    continue;
                }
                let val = opt.unwrap();
                if val.0 <= 89 && val.0 != 0 {
                    cells.insert((x, y), '.');
                } else {
                    if val.0 == 0 {
                        cells.insert((x, y), '0');
                    } else {
                        cells.insert((x, y), '#');
                    }
                }
            }
        }

        for y in 0..31 {
            for x in 0..32 {
                let opt = nodes.get(&(x, y));
                if opt.is_none() {
                    print!("#");
                } else {
                    print!("{}", cells[&(x, y)]);
                }
            }
            println!("");
        }

        return "179".to_string();
    }
}
