use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D25;

impl Solve for D25 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut j = 0;
        loop {
            let mut output: Vec<i32> = vec![];
            let (mut registers, commands) = setup(j);

            let mut instructions: Vec<Vec<String>> = vec![];
            for input in inputs.clone() {
                let mut temp: Vec<String> = input.split(" ").map(|f| f.to_string()).collect();
                if temp.len() == 2 {
                    temp.push("".to_string());
                }
                instructions.push(temp);
            }

            let mut i: i32 = 0;
            while i < instructions.len().try_into().unwrap() {
                if output.len() == 8 {
                    break;
                }
                let idx: usize = i.try_into().unwrap();
                let instruction: Vec<String> = instructions[idx].clone();

                let command = commands[&instruction[0]];
                i = command(
                    i,
                    &mut instructions,
                    &mut registers,
                    &mut output,
                    instruction[1].clone(),
                    instruction[2].clone(),
                );
            }
            if output == vec![0, 1, 0, 1, 0, 1, 0, 1] {
                break;
            }
            j += 1;
        }

        return j.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        return input.first().unwrap().to_string();
    }
}

fn setup(
    a: i32,
) -> (
    HashMap<String, i32>,
    HashMap<
        String,
        fn(
            i32,
            &mut Vec<Vec<String>>,
            &mut HashMap<String, i32>,
            &mut Vec<i32>,
            String,
            String,
        ) -> i32,
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
               _: &mut Vec<i32>,
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
               _: &mut Vec<i32>,
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
               _: &mut Vec<i32>,
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
               _: &mut Vec<i32>,
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
               _: &mut Vec<i32>,
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

    let out = |id: i32,
               _: &mut Vec<Vec<String>>,
               reg: &mut HashMap<String, i32>,
               out: &mut Vec<i32>,
               x: String,
               _: String|
     -> i32 {
        let value = match x.parse() {
            Ok(a) => a,
            Err(_) => reg[&x],
        };
        out.push(value);
        return id + 1;
    };

    let commands: HashMap<
        String,
        fn(
            i32,
            &mut Vec<Vec<String>>,
            &mut HashMap<String, i32>,
            &mut Vec<i32>,
            String,
            String,
        ) -> i32,
    > = HashMap::from([
        (
            "cpy".to_string(),
            cpy as fn(
                i32,
                &mut Vec<Vec<String>>,
                &mut HashMap<String, i32>,
                &mut Vec<i32>,
                String,
                String,
            ) -> i32,
        ),
        (
            "inc".to_string(),
            inc as fn(
                i32,
                &mut Vec<Vec<String>>,
                &mut HashMap<String, i32>,
                &mut Vec<i32>,
                String,
                String,
            ) -> i32,
        ),
        (
            "dec".to_string(),
            dec as fn(
                i32,
                &mut Vec<Vec<String>>,
                &mut HashMap<String, i32>,
                &mut Vec<i32>,
                String,
                String,
            ) -> i32,
        ),
        (
            "jnz".to_string(),
            jnz as fn(
                i32,
                &mut Vec<Vec<String>>,
                &mut HashMap<String, i32>,
                &mut Vec<i32>,
                String,
                String,
            ) -> i32,
        ),
        (
            "tgl".to_string(),
            tgl as fn(
                i32,
                &mut Vec<Vec<String>>,
                &mut HashMap<String, i32>,
                &mut Vec<i32>,
                String,
                String,
            ) -> i32,
        ),
        (
            "out".to_string(),
            out as fn(
                i32,
                &mut Vec<Vec<String>>,
                &mut HashMap<String, i32>,
                &mut Vec<i32>,
                String,
                String,
            ) -> i32,
        ),
    ]);
    return (registers, commands);
}
