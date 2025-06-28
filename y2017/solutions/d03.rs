use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D03;

impl Solve for D03 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let limit: i32 = input.first().unwrap().parse().unwrap();
        let containing_square;
        let mut coord_root = -1;
        let mut i = 1;
        loop {
            if i % 2 == 0 {
                i += 1;
                continue;
            }
            let square = i * i;
            coord_root += 1;
            if square > limit {
                containing_square = (i - 1) * (i - 1);
                break;
            }
            i += 1;
        }

        let diff = containing_square - limit;

        return (coord_root + coord_root - diff - 1).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let limit: i32 = input.first().unwrap().parse().unwrap();

        let mut map: HashMap<(i32, i32), i32> = HashMap::new();

        let mut i = 1;
        let mut base = 1;
        let mut x = 0;
        let mut y = 0;
        let mut quad = 0;
        let ans;
        loop {
            if i == base * base {
                let adj_sum = adjacent_sum(&mut map, x, y);

                if adj_sum > limit {
                    ans = adj_sum;
                    break;
                }
                map.insert((x, y), adj_sum);
                x += 1;
                i += 1;
                base += 2;
                quad = 0;
                continue;
            }

            let adj_sum = adjacent_sum(&mut map, x, y);

            if adj_sum > limit {
                ans = adj_sum;
                break;
            }

            map.insert((x, y), adj_sum);

            quad = next_quad(quad, x, y);

            (x, y) = next_coords(x, y, quad);

            i += 1;
        }

        return ans.to_string();
    }
}

fn next_coords(x: i32, y: i32, quad: usize) -> (i32, i32) {
    let quads = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    return (x + quads[quad].0, y + quads[quad].1);
}

fn adjacent_sum(map: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) -> i32 {
    if x == 0 && y == 0 {
        return 1;
    }
    let dirs = [
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let mut sum = 0;
    for dir in dirs {
        match map.get(&(x + dir.0, y + dir.1)) {
            Some(a) => sum += a,
            None => {}
        }
    }
    return sum;
}

fn next_quad(quad: usize, x: i32, y: i32) -> usize {
    if x > 0 && y > 0 && x == y {
        return 1;
    }
    if x < 0 && y > 0 && -x == y {
        return 2;
    }
    if x < 0 && y < 0 && x == y {
        return 3;
    }
    if x > 0 && y < 0 && x == -y {
        return 0;
    }

    return quad;
}
