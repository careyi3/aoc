use std::collections::HashMap;

pub trait Solve {
    fn part1(input: String, path: &String) -> String;
    fn part2(input: String, path: &String) -> String;
    fn solve(part: i32, input: String, path: String) -> String {
        match part {
            1 => {
                return Self::part1(input, &path);
            }
            2 => {
                return Self::part2(input, &path);
            }
            _ => {
                panic!()
            }
        }
    }
}

pub trait RunDay {
    fn fetch_days() -> HashMap<i32, fn(i32, String, String) -> String>;
    fn run_day(day: i32, part: i32, input: String, path: String) -> String {
        let days: HashMap<i32, fn(i32, String, String) -> String> = Self::fetch_days();
        let func = days.get(&day).unwrap();
        return func(part, input, path);
    }
}
