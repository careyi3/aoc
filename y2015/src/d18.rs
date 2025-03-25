use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D18;

impl Solve for D18 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut lights: HashMap<(i32, i32), i32> = HashMap::new();
        for (y, line) in inputs.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    lights.insert((x as i32, y as i32), 1);
                } else {
                    lights.insert((x as i32, y as i32), 0);
                }
            }
        }

        for _ in 0..100 {
            let mut temp_lights: HashMap<(i32, i32), i32> = HashMap::new();

            for coord in lights.keys() {
                let lit = lit(&lights, coord);
                if lights[coord] == 1 {
                    if lit == 2 || lit == 3 {
                        temp_lights.insert(*coord, 1);
                    } else {
                        temp_lights.insert(*coord, 0);
                    }
                }

                if lights[coord] == 0 {
                    if lit == 3 {
                        temp_lights.insert(*coord, 1);
                    } else {
                        temp_lights.insert(*coord, 0);
                    }
                }
            }

            lights = temp_lights;
        }

        return lights.values().into_iter().sum::<i32>().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut lights: HashMap<(i32, i32), i32> = HashMap::new();
        for (y, line) in inputs.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    lights.insert((x as i32, y as i32), 1);
                } else {
                    lights.insert((x as i32, y as i32), 0);
                }
            }
        }

        lights.insert((0, 0), 1);
        lights.insert((99, 99), 1);
        lights.insert((0, 99), 1);
        lights.insert((99, 0), 1);

        for _ in 0..100 {
            let mut temp_lights: HashMap<(i32, i32), i32> = HashMap::new();

            for coord in lights.keys() {
                let lit = lit(&lights, coord);
                if lights[coord] == 1 {
                    if lit == 2 || lit == 3 {
                        temp_lights.insert(*coord, 1);
                    } else {
                        temp_lights.insert(*coord, 0);
                    }
                }

                if lights[coord] == 0 {
                    if lit == 3 {
                        temp_lights.insert(*coord, 1);
                    } else {
                        temp_lights.insert(*coord, 0);
                    }
                }
            }

            lights = temp_lights;
            lights.insert((0, 0), 1);
            lights.insert((99, 99), 1);
            lights.insert((0, 99), 1);
            lights.insert((99, 0), 1);
        }

        return lights.values().into_iter().sum::<i32>().to_string();
    }
}

fn lit(lights: &HashMap<(i32, i32), i32>, coords: &(i32, i32)) -> i32 {
    let mut lit = 0;

    if lights.contains_key(&(coords.0 + 1, coords.1 + 1)) {
        lit += lights[&(coords.0 + 1, coords.1 + 1)]
    }
    if lights.contains_key(&(coords.0 + 1, coords.1)) {
        lit += lights[&(coords.0 + 1, coords.1)]
    }
    if lights.contains_key(&(coords.0, coords.1 + 1)) {
        lit += lights[&(coords.0, coords.1 + 1)]
    }
    if lights.contains_key(&(coords.0 - 1, coords.1 - 1)) {
        lit += lights[&(coords.0 - 1, coords.1 - 1)]
    }
    if lights.contains_key(&(coords.0 - 1, coords.1)) {
        lit += lights[&(coords.0 - 1, coords.1)]
    }
    if lights.contains_key(&(coords.0, coords.1 - 1)) {
        lit += lights[&(coords.0, coords.1 - 1)]
    }
    if lights.contains_key(&(coords.0 + 1, coords.1 - 1)) {
        lit += lights[&(coords.0 + 1, coords.1 - 1)]
    }
    if lights.contains_key(&(coords.0 - 1, coords.1 + 1)) {
        lit += lights[&(coords.0 - 1, coords.1 + 1)]
    }

    return lit;
}
