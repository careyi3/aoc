use std::collections::HashSet;
use utils::{file_reader, harness::Solve};

pub struct D01;

impl Solve for D01 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let instructions: Vec<String> = input[0].split(", ").map(|x| x.to_string()).collect();
        let mut current_dir: usize = 1;
        let dirs = [-1, 1, 1, -1];
        let mut x = 0;
        let mut y = 0;
        for instruction in instructions {
            let (dir, dist) = instruction.split_at(1);
            let steps: i32 = dist.parse().unwrap();
            if dir == "R" {
                current_dir += 1;
                if current_dir == 4 {
                    current_dir = 0
                };
            } else {
                if current_dir == 0 {
                    current_dir = 3;
                } else {
                    current_dir -= 1;
                }
            }
            if current_dir == 0 || current_dir == 2 {
                x += dirs[current_dir] * steps;
            } else {
                y += dirs[current_dir] * steps;
            }
        }

        return (x.abs() + y.abs()).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let instructions: Vec<String> = input[0].split(", ").map(|x| x.to_string()).collect();
        let mut current_dir: usize = 1;
        let dirs = [-1, 1, 1, -1];
        let mut x = 0;
        let mut y = 0;
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        'outer: for instruction in instructions {
            let (dir, dist) = instruction.split_at(1);
            let steps: i32 = dist.parse().unwrap();
            if dir == "R" {
                current_dir += 1;
                if current_dir == 4 {
                    current_dir = 0
                };
            } else {
                if current_dir == 0 {
                    current_dir = 3;
                } else {
                    current_dir -= 1;
                }
            }
            if current_dir == 0 || current_dir == 2 {
                let new_x = x + dirs[current_dir] * steps;
                if x > new_x {
                    for i in (new_x + 1..x).rev() {
                        if visited.contains(&(i, y)) {
                            x = i;
                            break 'outer;
                        } else {
                            visited.insert((i, y));
                        }
                    }
                } else {
                    for i in x..new_x {
                        if visited.contains(&(i, y)) {
                            x = i;
                            break 'outer;
                        } else {
                            visited.insert((i, y));
                        }
                    }
                }
                x = new_x;
            } else {
                let new_y = y + dirs[current_dir] * steps;
                if y > new_y {
                    for i in (new_y + 1..y).rev() {
                        if visited.contains(&(x, i)) {
                            y = i;
                            break 'outer;
                        } else {
                            visited.insert((x, i));
                        }
                    }
                } else {
                    for i in y..new_y {
                        if visited.contains(&(x, i)) {
                            y = i;
                            break 'outer;
                        } else {
                            visited.insert((x, i));
                        }
                    }
                }
                y = new_y;
            }
        }

        return (x.abs() + y.abs()).to_string();
    }
}
