use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;
use utils::{file_reader, harness::SolveResult};

pub struct D09;

impl SolveResult for D09 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut points: HashSet<(i64, i64)> = HashSet::new();
        for line in input {
            let coords: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
            points.insert((coords[0], coords[1]));
        }

        let mut largest = 0;
        for combo in points.iter().combinations(2) {
            let a = combo[0];
            let b = combo[1];

            let area = ((b.0 - a.0).abs() + 1) * ((b.1 - a.1).abs() + 1);
            if area > largest {
                largest = area;
            }
        }

        return Ok(largest.to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let input = file_reader::read_lines(path);

        let mut points: Vec<(i64, i64)> = vec![];
        for line in input {
            let coords: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
            points.push((coords[0], coords[1]));
        }

        let mut polygon_path: HashSet<(i64, i64)> = HashSet::new();
        for n in 0..points.len() {
            let start = points[n];
            let end = points[(n + 1) % points.len()];

            if start.0 == end.0 {
                for y in create_range_boxed(start.1, end.1) {
                    polygon_path.insert((start.0, y));
                }
            }
            if start.1 == end.1 {
                for x in create_range_boxed(start.0, end.0) {
                    polygon_path.insert((x, start.1));
                }
            }
        }

        let mut rectangles: Vec<((i64, i64), (i64, i64), i64)> = points
            .iter()
            .combinations(2)
            .map(|combo| {
                let a = *combo[0];
                let b = *combo[1];
                let area = ((b.0 - a.0).abs() + 1) * ((b.1 - a.1).abs() + 1);
                (a, b, area)
            })
            .collect();

        rectangles.sort_by(|x, y| y.2.cmp(&x.2));

        for (a, b, area) in rectangles {
            if rectangle_valid(&a, &b, &polygon_path) {
                return Ok(area.to_string());
            }
        }

        return Ok("0".to_string());
    }
}

fn create_range_boxed(a: i64, b: i64) -> Box<dyn Iterator<Item = i64>> {
    if a <= b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}

pub fn rectangle_valid(a: &(i64, i64), b: &(i64, i64), poly: &HashSet<(i64, i64)>) -> bool {
    let min_x = a.0.min(b.0);
    let max_x = a.0.max(b.0);
    let min_y = a.1.min(b.1);
    let max_y = a.1.max(b.1);

    for point in poly {
        let (x, y) = *point;
        if x > min_x && x < max_x && y > min_y && y < max_y {
            return false;
        }
    }

    true
}
