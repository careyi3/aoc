use utils::{file_reader, harness::Solve};

pub struct D03;

impl Solve for D03 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut valid = 0;
        for input in inputs {
            let sides: Vec<i32> = input
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let a = sides[0];
            let b = sides[1];
            let c = sides[2];

            if a + b > c && a + c > b && c + b > a {
                valid += 1;
            }
        }

        return valid.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut valid = 0;
        let mut a: Vec<i32> = vec![];
        let mut b: Vec<i32> = vec![];
        let mut c: Vec<i32> = vec![];
        for input in inputs {
            let sides: Vec<i32> = input
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            a.push(sides[0]);
            b.push(sides[1]);
            c.push(sides[2]);
        }

        let sides: Vec<i32> = vec![a, b, c].concat();

        let mut i = 0;
        while i < sides.len() {
            let a = sides[i];
            let b = sides[i + 1];
            let c = sides[i + 2];

            if a + b > c && a + c > b && c + b > a {
                valid += 1;
            }

            i += 3;
        }

        return valid.to_string();
    }
}
