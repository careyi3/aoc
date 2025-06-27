use itertools::Itertools;

use utils::{file_reader, harness::Solve};

pub struct D02;

impl Solve for D02 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut sum = 0;
        for input in inputs {
            let vals: Vec<i32> = input
                .split("	")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let mut highest = 0;
            let mut lowest = 1000000;
            for val in vals {
                if val > highest {
                    highest = val;
                }
                if val < lowest {
                    lowest = val;
                }
            }
            sum += highest - lowest;
        }

        return sum.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut sum = 0;
        for input in inputs {
            let vals: Vec<i32> = input
                .split("	")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            for comb in vals.iter().combinations(2) {
                if comb[0] < comb[1] {
                    if comb[1] % comb[0] == 0 {
                        sum += comb[1] / comb[0];
                        break;
                    }
                } else {
                    if comb[0] % comb[1] == 0 {
                        sum += comb[0] / comb[1];
                        break;
                    }
                }
            }
        }

        return sum.to_string();
    }
}
