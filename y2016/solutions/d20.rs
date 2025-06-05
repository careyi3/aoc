use utils::{file_reader, harness::Solve};

pub struct D20;

impl Solve for D20 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut ranges: Vec<(i64, i64)> = vec![];
        for input in inputs {
            let vals: Vec<i64> = input
                .split("-")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            ranges.push((vals[0], vals[1]));
        }
        ranges.sort_by(|x, y| x.0.cmp(&y.0));

        let mut new_ranges: Vec<(i64, i64)> = vec![];
        let mut low = 0;
        let mut high = 0;
        for (a, b) in ranges {
            if a > high + 1 {
                new_ranges.push((low, high));
                low = a;
                high = b;
                continue;
            }
            if b > high {
                high = b;
            }
        }

        return (new_ranges[0].1 + 1).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut ranges: Vec<(i64, i64)> = vec![];
        for input in inputs {
            let vals: Vec<i64> = input
                .split("-")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            ranges.push((vals[0], vals[1]));
        }
        ranges.sort_by(|x, y| x.0.cmp(&y.0));

        let mut new_ranges: Vec<(i64, i64)> = vec![];
        let mut low = 0;
        let mut high = 0;
        for (a, b) in ranges {
            if a > high + 1 {
                new_ranges.push((low, high));
                low = a;
                high = b;
                continue;
            }
            if b > high {
                high = b;
            }
        }

        let mut sum = 1;
        for i in 0..new_ranges.len() - 1 {
            let (_, b1) = new_ranges[i];
            let (a2, _) = new_ranges[i + 1];
            let diff = a2 - (b1 + 1);
            sum += diff;
        }

        return sum.to_string();
    }
}
