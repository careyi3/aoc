use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D07;

impl Solve for D07 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut commands: HashMap<String, Vec<String>> = HashMap::new();
        let mut cache: HashMap<String, u16> = HashMap::new();

        for input in inputs {
            let tokens: Vec<String> = input.split(' ').map(|x| x.to_string()).collect();

            let key = format!("{}", tokens.last().unwrap());
            commands.insert(key, tokens);
        }

        return calc(&commands, &"a".to_string(), &mut cache).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut commands: HashMap<String, Vec<String>> = HashMap::new();
        let mut cache: HashMap<String, u16> = HashMap::new();

        for input in inputs {
            let tokens: Vec<String> = input.split(' ').map(|x| x.to_string()).collect();

            let key = format!("{}", tokens.last().unwrap());
            commands.insert(key, tokens);
        }

        return calc(&commands, &"a".to_string(), &mut cache).to_string();
    }
}

fn parse_or_compute(
    command: &String,
    commands: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    return match command.parse() {
        Ok(n) => n,
        Err(_) => calc(commands, &command, cache),
    };
}

fn calc(
    commands: &HashMap<String, Vec<String>>,
    command: &String,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    let tokens = &commands[command];

    if cache.contains_key(command) {
        return cache[command];
    }

    //NOT
    if tokens[0] == "NOT" {
        let a = parse_or_compute(&tokens[1], commands, cache);
        let ans = !a;
        cache.insert(command.clone(), ans);
        return ans;
    }

    //AND
    if tokens[1] == "AND" {
        let a = parse_or_compute(&tokens[0], commands, cache);
        let b = parse_or_compute(&tokens[2], commands, cache);
        let ans = a & b;
        cache.insert(command.clone(), ans);
        return ans;
    }

    //LSHIFT
    if tokens[1] == "LSHIFT" {
        let a = parse_or_compute(&tokens[0], commands, cache);
        let b = parse_or_compute(&tokens[2], commands, cache);
        let ans = a << b;
        cache.insert(command.clone(), ans);
        return ans;
    }

    //RSHIFT
    if tokens[1] == "RSHIFT" {
        let a = parse_or_compute(&tokens[0], commands, cache);
        let b = parse_or_compute(&tokens[2], commands, cache);
        let ans = a >> b;
        cache.insert(command.clone(), ans);
        return ans;
    }

    // OR
    if tokens[1] == "OR" {
        let a = parse_or_compute(&tokens[0], commands, cache);
        let b = parse_or_compute(&tokens[2], commands, cache);
        let ans = a | b;
        cache.insert(command.clone(), ans);
        return ans;
    }

    //CONST
    let ans = parse_or_compute(&tokens[0], commands, cache);
    cache.insert(command.clone(), ans);
    return ans;
}
