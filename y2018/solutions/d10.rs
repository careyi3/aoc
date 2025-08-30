use fancy_regex::Regex;
use std::{collections::HashSet, i32};
use utils::{file_reader, harness::Solve};

pub struct D10;

impl Solve for D10 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut points: Vec<(i32, i32, i32, i32)> = vec![];
        for line in inputs {
            points.push(parse_line(&line).unwrap());
        }
        let mut map: HashSet<(i32, i32)>;
        let mut i = 0;
        let iters = 10618;

        loop {
            if i > iters {
                break;
            }
            let mut x_min = i32::MAX;
            let mut x_max = i32::MIN;

            let mut y_min = i32::MAX;
            let mut y_max = i32::MIN;
            map = HashSet::new();
            for (x, y, vx, vy) in points.iter_mut() {
                map.insert((*x + *vx, *y + *vy));
                *x += *vx;
                *y += *vy;
                if *x > x_max {
                    x_max = *x;
                }
                if *y > y_max {
                    y_max = *y;
                }
                if *x < x_min {
                    x_min = *x;
                }
                if *y < y_min {
                    y_min = *y;
                }
            }
            if y_max - y_min < 40 {
                for y in y_min - 2..y_max + 2 {
                    let mut xs: String = "".to_string();
                    for x in x_min - 2..x_max + 3 {
                        if map.contains(&(x, y)) {
                            xs.push('#');
                        } else {
                            xs.push('.');
                        }
                    }
                    println!("{}", xs);
                }
                println!("");
            }
            i += 1;
        }

        return (iters + 1).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut points: Vec<(i32, i32, i32, i32)> = vec![];
        for line in inputs {
            points.push(parse_line(&line).unwrap());
        }
        let mut map: HashSet<(i32, i32)>;
        let mut i = 0;
        let iters = 10618;

        loop {
            if i > iters {
                break;
            }
            let mut x_min = i32::MAX;
            let mut x_max = i32::MIN;

            let mut y_min = i32::MAX;
            let mut y_max = i32::MIN;
            map = HashSet::new();
            for (x, y, vx, vy) in points.iter_mut() {
                map.insert((*x + *vx, *y + *vy));
                *x += *vx;
                *y += *vy;
                if *x > x_max {
                    x_max = *x;
                }
                if *y > y_max {
                    y_max = *y;
                }
                if *x < x_min {
                    x_min = *x;
                }
                if *y < y_min {
                    y_min = *y;
                }
            }
            if y_max - y_min < 40 {
                for y in y_min - 2..y_max + 2 {
                    let mut xs: String = "".to_string();
                    for x in x_min - 2..x_max + 3 {
                        if map.contains(&(x, y)) {
                            xs.push('#');
                        } else {
                            xs.push('.');
                        }
                    }
                    println!("{}", xs);
                }
                println!("");
            }
            i += 1;
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
