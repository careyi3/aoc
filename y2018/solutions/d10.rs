use fancy_regex::Regex;
use std::collections::HashSet;
use utils::{file_reader, harness::Solve};

pub struct D10;

impl Solve for D10 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut points: Vec<(i32, i32, i32, i32)> = vec![];
        for line in inputs {
            points.push(parse_line(&line).unwrap());
        }
        let mut map: HashSet<(i32, i32)> = HashSet::new();
        let mut i = 0;
        let iters = 10618;
        loop {
            if i > iters {
                break;
            }
            map = HashSet::new();
            for (x, y, vx, vy) in points.iter_mut() {
                map.insert((*x + *vx, *y + *vy));
                *x += *vx;
                *y += *vy;
            }
            i += 1;
        }

        for y in 190..204 {
            let mut xs: String = "".to_string();
            for x in 135..201 {
                if map.contains(&(x, y)) {
                    xs.push('#');
                } else {
                    xs.push('.');
                }
            }
            println!("{}", xs);
        }

        return (iters + 1).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut points: Vec<(i32, i32, i32, i32)> = vec![];
        for line in inputs {
            points.push(parse_line(&line).unwrap());
        }
        let mut map: HashSet<(i32, i32)> = HashSet::new();
        let mut i = 0;
        let iters = 10618;
        loop {
            if i > iters {
                break;
            }
            map = HashSet::new();
            for (x, y, vx, vy) in points.iter_mut() {
                map.insert((*x + *vx, *y + *vy));
                *x += *vx;
                *y += *vy;
            }
            i += 1;
        }

        for y in 190..204 {
            let mut xs: String = "".to_string();
            for x in 135..201 {
                if map.contains(&(x, y)) {
                    xs.push('#');
                } else {
                    xs.push('.');
                }
            }
            println!("{}", xs);
        }

        return (iters + 1).to_string();
    }
}

fn parse_line(s: &str) -> Option<(i32, i32, i32, i32)> {
    let re =
        Regex::new(r"^position=<\s*(-?\d+),\s*(-?\d+)>\s*velocity=<\s*(-?\d+),\s*(-?\d+)>\s*$")
            .ok()?;
    let caps = re.captures(s).ok().flatten()?;
    Some((
        caps.get(1)?.as_str().parse().ok()?,
        caps.get(2)?.as_str().parse().ok()?,
        caps.get(3)?.as_str().parse().ok()?,
        caps.get(4)?.as_str().parse().ok()?,
    ))
}
