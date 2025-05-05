use utils::{file_reader, harness::Solve};

pub struct D07;

impl Solve for D07 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut count = 0;
        'outer: for input in inputs {
            let words: Vec<String> = input.split(['[', ']']).map(|x| x.to_string()).collect();
            let mut has_repeat = false;
            for (idx, word) in words.iter().enumerate() {
                if (idx + 1) % 2 == 0 {
                    if has_repeat_pattern(word) {
                        continue 'outer;
                    }
                } else {
                    if has_repeat_pattern(word) {
                        has_repeat = true;
                    }
                }
            }
            if has_repeat {
                count += 1;
            }
        }

        return count.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut count = 0;
        'outer: for input in inputs {
            let words: Vec<String> = input.split(['[', ']']).map(|x| x.to_string()).collect();
            let mut outsides: Vec<String> = vec![];
            let mut insides: Vec<String> = vec![];
            for (idx, word) in words.iter().enumerate() {
                let mut patterns = fetch_pattern(word);
                if (idx + 1) % 2 == 0 {
                    outsides.append(&mut patterns);
                } else {
                    insides.append(&mut patterns);
                }
            }
            for outer in &outsides {
                for inner in &insides {
                    if outer.as_bytes()[0] == inner.as_bytes()[1]
                        && outer.as_bytes()[1] == inner.as_bytes()[0]
                    {
                        count += 1;
                        continue 'outer;
                    }
                }
            }
        }

        return count.to_string();
    }
}

fn has_repeat_pattern(word: &String) -> bool {
    let mut buffer = vec![];
    for letter in word.chars() {
        if buffer.len() == 3 {
            buffer.push(letter);
            if buffer[0] == buffer[3] && buffer[1] == buffer[2] && buffer[0] != buffer[1] {
                return true;
            }
            buffer.remove(0);
        } else {
            buffer.push(letter);
        }
    }

    return false;
}

fn fetch_pattern(word: &String) -> Vec<String> {
    let mut patterns = vec![];
    let mut buffer = vec![];
    for letter in word.chars() {
        if buffer.len() == 2 {
            buffer.push(letter);
            if buffer[0] == buffer[2] && buffer[0] != buffer[1] {
                patterns.push(buffer.iter().collect());
            }
            buffer.remove(0);
        } else {
            buffer.push(letter);
        }
    }
    return patterns;
}
