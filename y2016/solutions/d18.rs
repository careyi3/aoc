use utils::{file_reader, harness::Solve};

pub struct D18;

impl Solve for D18 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let input = inputs.first().unwrap();

        let mut line: Vec<char> = input
            .chars()
            .map(|x| if x == '.' { '0' } else { '1' })
            .collect();
        let mut lines: Vec<Vec<char>> = vec![];
        lines.push(line.clone());

        for _ in 0..39 {
            let mut new_line: Vec<char> = vec![];
            let mut window: [char; 3] = ['0', '0', '0'];
            for i in 0..line.len() {
                if i == 0 {
                    window[0] = '0';
                    window[1] = line[i];
                    window[2] = line[i + 1];
                } else if i == line.len() - 1 {
                    window[0] = line[i - 1];
                    window[1] = line[i];
                    window[2] = '0';
                } else {
                    window[0] = line[i - 1];
                    window[1] = line[i];
                    window[2] = line[i + 1];
                }
                let bits: String = window.iter().collect();
                let num = isize::from_str_radix(&bits, 2).unwrap();
                if [6, 3, 4, 1].contains(&num) {
                    new_line.push('1');
                } else {
                    new_line.push('0');
                }
            }
            lines.push(new_line.clone());
            line = new_line;
        }

        let mut count = 0;

        for line in lines {
            for char in line {
                if char == '0' {
                    count += 1;
                }
            }
        }

        return count.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let input = inputs.first().unwrap();

        let mut line: Vec<char> = input
            .chars()
            .map(|x| if x == '.' { '0' } else { '1' })
            .collect();
        let mut lines: Vec<Vec<char>> = vec![];
        lines.push(line.clone());

        for _ in 0..399999 {
            let mut new_line: Vec<char> = vec![];
            let mut window: [char; 3] = ['0', '0', '0'];
            for i in 0..line.len() {
                if i == 0 {
                    window[0] = '0';
                    window[1] = line[i];
                    window[2] = line[i + 1];
                } else if i == line.len() - 1 {
                    window[0] = line[i - 1];
                    window[1] = line[i];
                    window[2] = '0';
                } else {
                    window[0] = line[i - 1];
                    window[1] = line[i];
                    window[2] = line[i + 1];
                }
                let bits: String = window.iter().collect();
                let num = isize::from_str_radix(&bits, 2).unwrap();
                if [6, 3, 4, 1].contains(&num) {
                    new_line.push('1');
                } else {
                    new_line.push('0');
                }
            }
            lines.push(new_line.clone());
            line = new_line;
        }

        let mut count = 0;

        for line in lines {
            for char in line {
                if char == '0' {
                    count += 1;
                }
            }
        }

        return count.to_string();
    }
}
