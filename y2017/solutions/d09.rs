use utils::{file_reader, harness::Solve};

pub struct D09;

impl Solve for D09 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        let mut total = 0;
        let mut depth = 0;
        let mut in_garbage = false;
        let chars: Vec<char> = input.first().unwrap().chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let char = chars[i];
            if char == '{' && !in_garbage {
                depth += 1;
            }
            if char == '}' && !in_garbage {
                total += depth;
                depth -= 1;
            }
            if char == '<' {
                in_garbage = true;
            }
            if char == '>' {
                in_garbage = false;
            }
            if char == '!' {
                i += 1;
            }
            i += 1;
        }

        return total.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        let mut total = 0;
        let mut in_garbage = false;
        let chars: Vec<char> = input.first().unwrap().chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let char = chars[i];
            if char == '>' {
                in_garbage = false;
            }
            if in_garbage {
                total += 1;
            }
            if char == '<' {
                in_garbage = true;
            }
            if char == '!' {
                if in_garbage {
                    total -= 1;
                }
                i += 1;
            }
            i += 1;
        }

        return total.to_string();
    }
}
