use utils::{file_reader, harness::Solve};

pub struct D16;

impl Solve for D16 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_line(path);
        //let mut programs = ['a', 'b', 'c', 'd', 'e'];
        let mut programs = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        ];

        for m in input.split(",").map(|x| x.to_string()) {
            let mut command: char = ' ';
            let mut params: String = "".to_string();
            for (idx, char) in m.chars().enumerate() {
                if idx == 0 {
                    command = char;
                } else {
                    params.push(char);
                }
            }
            execute(command, params, &mut programs)
        }

        return programs.iter().collect();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_line(path);
        //let mut programs = ['a', 'b', 'c', 'd', 'e'];
        let mut programs = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        ];
        let mut patterns: Vec<String> = vec![];
        for _ in 0..42 {
            // Found by trial and error to have a repeating cycle length from the start of 42.
            for m in input.split(",").map(|x| x.to_string()) {
                let mut command: char = ' ';
                let mut params: String = "".to_string();
                for (idx, char) in m.chars().enumerate() {
                    if idx == 0 {
                        command = char;
                    } else {
                        params.push(char);
                    }
                }
                execute(command, params, &mut programs)
            }
            let pattern: String = programs.iter().collect();
            patterns.push(pattern);
        }

        let ans = &patterns[(1000000000 % 42) - 1];

        return ans.clone();
    }
}

fn execute(command: char, params: String, programs: &mut [char; 16]) {
    match command {
        's' => {
            let size: usize = params.parse().unwrap();
            programs.rotate_right(size);
        }
        'x' => {
            let indexes: Vec<usize> = params
                .split("/")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let temp_a = programs[indexes[0]];
            let temp_b = programs[indexes[1]];

            programs[indexes[0]] = temp_b;
            programs[indexes[1]] = temp_a;
        }
        'p' => {
            let chars: Vec<u8> = params
                .split("/")
                .map(|x| *x.as_bytes().first().unwrap())
                .collect();

            let mut a_index = 0;
            let mut b_index = 0;

            for (idx, c) in programs.iter().enumerate() {
                if *c == chars[0] as char {
                    a_index = idx;
                }
                if *c == chars[1] as char {
                    b_index = idx;
                }
            }

            let temp_a = programs[a_index];
            let temp_b = programs[b_index];

            programs[a_index] = temp_b;
            programs[b_index] = temp_a;
        }
        _ => {}
    }
}
