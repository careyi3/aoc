use utils::{file_reader, harness::Solve};

pub struct D10;

impl Solve for D10 {
    fn part1(_input: String, path: &String) -> String {
        let input = &file_reader::read_lines(path)[0];

        let mut s: String;
        let mut new_s: String = input.clone();
        new_s.push('.');
        for _ in 0..40 {
            let chars: Vec<char> = new_s.chars().collect();
            let mut last = chars[0];
            let mut count = 0;
            s = "".to_string();
            for char in new_s.chars() {
                let current = char;
                if current != last {
                    s.push_str(&count.to_string());
                    s.push(last);
                    count = 1;
                } else {
                    count += 1;
                }
                last = current;
            }

            new_s = s;
            new_s.push('.');
        }

        return (new_s.chars().count() - 1).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = &file_reader::read_lines(path)[0];

        let mut s: String;
        let mut new_s: String = input.clone();
        new_s.push('.');
        for _ in 0..50 {
            let chars: Vec<char> = new_s.chars().collect();
            let mut last = chars[0];
            let mut count = 0;
            s = "".to_string();
            for char in new_s.chars() {
                let current = char;
                if current != last {
                    s.push_str(&count.to_string());
                    s.push(last);
                    count = 1;
                } else {
                    count += 1;
                }
                last = current;
            }

            new_s = s;
            new_s.push('.');
        }

        return (new_s.chars().count() - 1).to_string();
    }
}
