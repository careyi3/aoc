use std::collections::VecDeque;

use anyhow::Result;
use utils::{file_reader, harness::SolveResult};

pub struct D05;

impl SolveResult for D05 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut ranges: Vec<Vec<i64>> = vec![];
        let mut ids: Vec<i64> = vec![];
        let mut reading_ranges = true;
        for line in input {
            if line == "" {
                reading_ranges = false;
                continue;
            }

            if reading_ranges {
                let range: Vec<i64> = line.split('-').map(|x| x.parse::<i64>().unwrap()).collect();
                ranges.push(range);
            } else {
                let id = line.parse::<i64>()?;
                ids.push(id);
            }
        }

        let mut fresh = 0;
        for id in ids {
            for range in &ranges {
                if id >= range[0] && id <= range[1] {
                    fresh += 1;
                    break;
                }
            }
        }

        return Ok(fresh.to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut ranges: VecDeque<Vec<i64>> = VecDeque::new();
        for line in input {
            if line == "" {
                break;
            }
            let range: Vec<i64> = line.split('-').map(|x| x.parse::<i64>().unwrap()).collect();
            ranges.push_back(range);
        }

        loop {
            let mut temp_ranges = ranges.clone();

            let to_test = temp_ranges.pop_back().unwrap();
            let mut reduced = false;
            for range in temp_ranges.iter_mut() {
                let a = to_test[0];
                let b = to_test[1];
                let c = range[0];
                let d = range[1];
                // A---B
                //     C---D
                if b == c {
                    range[0] = a;
                    range[1] = d;
                    reduced = true;
                    break;
                }
                // C---D
                //     A---B
                if a == d {
                    range[0] = c;
                    range[1] = b;
                    reduced = true;
                    break;
                }
                //     C---D
                //   A---B
                if c <= b && a <= c && c >= b {
                    range[0] = a;
                    range[1] = d;
                    reduced = true;
                    break;
                }
                // C---D
                //   A---B
                if c <= a && d >= a && d <= b {
                    range[0] = c;
                    range[1] = b;
                    reduced = true;
                    break;
                }
                //     C---D
                //   A------B
                if a <= c && b >= d {
                    range[0] = a;
                    range[1] = b;
                    reduced = true;
                    break;
                }
                // C-------D
                //   A---B
                if a >= c && b <= d {
                    range[0] = c;
                    range[1] = d;
                    reduced = true;
                    break;
                }
            }
            if !reduced {
                temp_ranges.push_front(to_test.clone());
            }
            ranges = temp_ranges;
            if ranges.len() == 2 {
                break;
            }
            if ranges.len() == 93 {
                break;
            }
        }

        let mut count = 0;
        for range in ranges {
            count += range[1] + 1 - range[0];
        }

        return Ok(count.to_string());
    }
}
