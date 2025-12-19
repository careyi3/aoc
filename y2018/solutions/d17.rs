use anyhow::Result;
use std::collections::{HashMap, HashSet};
use utils::{
    annimations::save_annimation_data, annimations::Frame, file_reader, harness::SolveResult,
};

pub struct D17;

impl SolveResult for D17 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);
        let mut map: HashMap<(i32, i32), char> = HashMap::new();

        let mut y_min = 1500;
        let mut y_max = 0;
        let mut x_min = 1500;
        let mut x_max = 0;
        for line in &input {
            let points = parse_line(line)?;
            for (x, y) in points {
                if y < y_min {
                    y_min = y;
                }
                if y > y_max {
                    y_max = y;
                }
                if x < x_min {
                    x_min = x;
                }
                if x > x_max {
                    x_max = x;
                }
                map.insert((x, y), '#');
            }
        }

        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut frames: Vec<Frame> = vec![];
        let mut step = 0;

        walk(
            &mut map,
            &mut visited,
            &mut frames,
            &mut step,
            (x_min, y_min),
            (x_max, y_max),
            500,
            0,
        );

        save_annimation_data(
            "2028_day_17".to_string(),
            200 as usize,
            200 as usize,
            frames,
            250.0,
        );

        let mut count = 0;
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if visited.contains(&(x, y)) {
                    count += 1;
                }
            }
        }

        return Ok(count.to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        return Ok(input.first().unwrap().to_string());
    }
}

fn parse_line(line: &String) -> Result<Vec<(i32, i32)>> {
    let split = line.split(", ").collect::<Vec<&str>>();
    let point = split[0].split("=").collect::<Vec<&str>>();
    let range = split[1].split("=").collect::<Vec<&str>>();

    let mut points: Vec<(i32, i32)> = vec![];
    if point[0] == "x" {
        let x = point[1].parse::<i32>()?;
        let ys: Vec<i32> = range[1]
            .split("..")
            .map(|y| y.parse::<i32>().unwrap())
            .collect();
        for y in ys[0]..=ys[1] {
            points.push((x, y));
        }
    } else {
        let y = point[1].parse::<i32>()?;
        let xs: Vec<i32> = range[1]
            .split("..")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        for x in xs[0]..=xs[1] {
            points.push((x, y));
        }
    }
    Ok(points)
}

fn generate_frame(
    map: &HashMap<(i32, i32), char>,
    visited: &HashSet<(i32, i32)>,
    width: i32,
    height: i32,
    current: (i32, i32),
    step: usize,
) -> Frame {
    let mut grid: Vec<Vec<u8>> = vec![];
    for y in current.1 - (width / 2)..current.1 + (width / 2) {
        let mut line = vec![];
        for x in current.0 - (height / 2)..current.0 + (height / 2) {
            if map.contains_key(&(x, y)) {
                if map[&(x, y)] == '#' {
                    line.push(1);
                } else {
                    line.push(4);
                }
                continue;
            }
            if visited.contains(&(x, y)) {
                line.push(5);
                continue;
            }
            line.push(0);
        }
        grid.push(line);
    }
    let highlighted = vec![(current.0 as usize, current.1 as usize)];
    return Frame {
        step,
        message: format!("Iteration: {}", step),
        grid,
        highlighted,
    };
}

fn walk(
    map: &mut HashMap<(i32, i32), char>,
    visited: &mut HashSet<(i32, i32)>,
    frames: &mut Vec<Frame>,
    step: &mut usize,
    min: (i32, i32),
    max: (i32, i32),
    x: i32,
    y: i32,
) {
    if y > max.1 {
        return;
    }
    if map.contains_key(&(x, y)) || visited.contains(&(x, y)) {
        return;
    }
    visited.insert((x, y));
    let frame = generate_frame(map, visited, 200, 200, (x, y), *step);
    *step += 1;
    frames.push(frame);
    for (idx, (xp, yp)) in [(0, 1), (-1, 0), (1, 0)].iter().enumerate() {
        if idx > 0 {
            if map.contains_key(&(x + xp, y + 1)) || map.contains_key(&(x, y + 1)) {
                walk(map, visited, frames, step, min, max, x + xp, y + yp);
            }
        } else {
            walk(map, visited, frames, step, min, max, x + xp, y + yp);
        }
        if inside_basin(map, x, y) {
            map.insert((x, y), '~');
        }
    }
}

fn inside_basin(map: &mut HashMap<(i32, i32), char>, x: i32, y: i32) -> bool {
    let mut left_x = x;
    let mut right_x = x;
    let mut inside = true;
    loop {
        if !map.contains_key(&(left_x, y + 1)) || !map.contains_key(&(right_x, y + 1)) {
            inside = false;
            break;
        }
        if map.contains_key(&(left_x, y)) && map.contains_key(&(right_x, y)) {
            break;
        }
        if !map.contains_key(&(left_x, y)) {
            left_x -= 1;
        }
        if !map.contains_key(&(right_x, y)) {
            right_x += 1;
        }
    }
    return inside;
}
