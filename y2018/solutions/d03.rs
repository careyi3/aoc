use fancy_regex::Regex;
use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D03;

impl Solve for D03 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut coords: HashMap<(i32, i32), i32> = HashMap::new();
        for line in inputs {
            let (_, left, top, width, height) = parse_line(&line);
            for x in left..left + width {
                for y in top..top + height {
                    coords.entry((x, y)).and_modify(|x| *x += 1).or_insert(1);
                }
            }
        }

        return coords
            .values()
            .filter(|x| **x > 1)
            .map(|a| *a)
            .count()
            .to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut coords: HashMap<(i32, i32), i32> = HashMap::new();
        for line in &inputs {
            let (_, left, top, width, height) = parse_line(&line);
            for x in left..left + width {
                for y in top..top + height {
                    coords.entry((x, y)).and_modify(|x| *x += 1).or_insert(1);
                }
            }
        }

        let mut ans: i32 = 0;
        for line in &inputs {
            let (id, left, top, width, height) = parse_line(&line);
            let mut sum = 0;
            for x in left..left + width {
                for y in top..top + height {
                    sum += coords[&(x, y)];
                }
            }
            if sum == width * height {
                ans = id;
                break;
            }
        }

        return ans.to_string();
    }
}

fn parse_line(line: &str) -> (i32, i32, i32, i32, i32) {
    let re = Regex::new(r"^#(\d+)\s*@\s*(\d+),(\d+):\s*(\d+)x(\d+)$").unwrap();
    let caps = re.captures(line).ok().flatten().unwrap();
    (
        caps.get(1).unwrap().as_str().parse().unwrap(),
        caps.get(2).unwrap().as_str().parse().unwrap(),
        caps.get(3).unwrap().as_str().parse().unwrap(),
        caps.get(4).unwrap().as_str().parse().unwrap(),
        caps.get(5).unwrap().as_str().parse().unwrap(),
    )
}
