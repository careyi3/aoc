use utils::{file_reader, harness::Solve};

pub struct D11;

impl Solve for D11 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let num = input.first().unwrap().parse::<i32>().unwrap();

        let mut max_power = 0;
        let mut coord = (0, 0);
        for y in 0..300 {
            for x in 0..300 {
                let power = grid_power(x, y, 3, num);
                if power > max_power {
                    max_power = power;
                    coord = (x, y);
                }
            }
        }

        return format!("{},{}", coord.0, coord.1).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let num = input.first().unwrap().parse::<i32>().unwrap();

        let mut max_power = 0;
        let mut coord = (0, 0, 0);
        for size in 3..32 {
            for y in 0..300 - size {
                for x in 0..300 - size {
                    let power = grid_power(x, y, size, num);
                    if power > max_power {
                        max_power = power;
                        coord = (x, y, size);
                    }
                }
            }
        }

        return format!("{},{},{}", coord.0, coord.1, coord.2).to_string();
    }
}

fn grid_power(x: i32, y: i32, size: i32, num: i32) -> i32 {
    let mut power = 0;
    for xp in x..x + size {
        for yp in y..y + size {
            power += calculate_power(xp, yp, num);
        }
    }
    return power;
}

fn calculate_power(x: i32, y: i32, num: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += num;
    power_level *= rack_id;
    power_level = (power_level / 100).rem_euclid(10);
    return power_level - 5;
}
