use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D06;

impl Solve for D06 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut points: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut x_max = 0;
        let mut y_max = 0;
        for (id, coords) in inputs.iter().enumerate() {
            let xy: Vec<i32> = coords
                .split(", ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            if xy[0] > x_max {
                x_max = xy[0];
            }
            if xy[1] > y_max {
                y_max = xy[1];
            }
            points.insert(id as i32, (xy[0], xy[1]));
        }

        let mut areas: HashMap<i32, i32> = HashMap::new();
        let mut edges: HashMap<i32, bool> = HashMap::new();
        for x in 0..x_max + 1 {
            for y in 0..y_max + 1 {
                let edge = x == 0 || y == 0 || x == x_max || y == y_max;
                let mut dists: HashMap<i32, Vec<i32>> = HashMap::new();
                for (point, (x_cord, y_cord)) in points.iter() {
                    let dist = (x_cord - x).abs() + (y_cord - y).abs();
                    dists
                        .entry(dist)
                        .and_modify(|v| v.push(*point))
                        .or_insert(vec![*point]);
                }
                let smallest = dists.keys().min().unwrap();
                if dists[smallest].len() == 1 {
                    let p = dists[smallest][0];
                    areas.entry(p).and_modify(|a| *a += 1).or_insert(1);
                    if edge {
                        edges.insert(p, true);
                    } else {
                        edges.entry(p).or_insert(edge);
                    }
                } else {
                    continue;
                }
            }
        }

        let mut largest = 0;
        for (point, area) in areas {
            if area > largest && !edges[&point] {
                largest = area;
            }
        }

        return largest.to_string();
    }

    fn part2(input: String, path: &String) -> String {
        let limit: i32;
        if input == "input".to_string() {
            limit = 10000;
        } else {
            limit = 32;
        }
        let inputs = file_reader::read_lines(path);
        let mut points: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut x_max = 0;
        let mut y_max = 0;
        for (id, coords) in inputs.iter().enumerate() {
            let xy: Vec<i32> = coords
                .split(", ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            if xy[0] > x_max {
                x_max = xy[0];
            }
            if xy[1] > y_max {
                y_max = xy[1];
            }
            points.insert(id as i32, (xy[0], xy[1]));
        }

        let mut area = 0;
        for x in 0..x_max + 1 {
            for y in 0..y_max + 1 {
                let mut dist = 0;
                for (_, (x_cord, y_cord)) in points.iter() {
                    dist += (x_cord - x).abs() + (y_cord - y).abs();
                }
                if dist < limit {
                    area += 1;
                }
            }
        }

        return area.to_string();
    }
}
