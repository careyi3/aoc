use utils::{file_reader, harness::Solve};

pub struct D25;

impl Solve for D25 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let line = input.first().unwrap();
        let nums: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        let mut value: i64 = 20151125;
        let mut i = 2;
        'outer: loop {
            let mut y = i;
            for x in 1..i + 1 {
                let temp = value * 252533;
                value = temp % 33554393;
                if x == nums[1] && y == nums[0] {
                    break 'outer;
                }
                y -= 1;
            }
            i += 1;
        }

        return value.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let line = input.first().unwrap();
        let nums: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        let mut value: i64 = 20151125;
        let mut i = 2;
        'outer: loop {
            let mut y = i;
            for x in 1..i + 1 {
                let temp = value * 252533;
                value = temp % 33554393;
                if x == nums[1] && y == nums[0] {
                    break 'outer;
                }
                y -= 1;
            }
            i += 1;
        }

        return value.to_string();
    }
}
