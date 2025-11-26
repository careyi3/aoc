use std::collections::HashMap;
use std::time::Instant;

pub trait SolveResult {
    fn part1(input: String, path: &String) -> anyhow::Result<String>;
    fn part2(input: String, path: &String) -> anyhow::Result<String>;
}

impl<T: SolveResult> Solve for T {
    fn part1(input: String, path: &String) -> String {
        T::part1(input, path).unwrap_or_else(|e| format!("Error: {}", e))
    }
    fn part2(input: String, path: &String) -> String {
        T::part2(input, path).unwrap_or_else(|e| format!("Error: {}", e))
    }
}

pub trait Solve {
    fn part1(input: String, path: &String) -> String;
    fn part2(input: String, path: &String) -> String;
    fn solve(part: i32, input: String, path: String) -> String {
        let now = Instant::now();
        let answer = match part {
            1 => Self::part1(input, &path),
            2 => Self::part2(input, &path),
            _ => panic!("Invalid part: {}. Must be 1 or 2.", part),
        };
        let elapsed = now.elapsed();
        println!("Runtime: {:.2?}", elapsed);
        answer
    }
}

pub trait RunDay {
    fn fetch_days() -> HashMap<String, fn(i32, String, String) -> String>;
    fn run_day(day: String, part: i32, input: String, path: String) -> String {
        let days: HashMap<String, fn(i32, String, String) -> String> = Self::fetch_days();
        let func = days.get(&day).unwrap();
        func(part, input, path)
    }
}

pub struct YearRunner {
    pub year: i32,
    pub runner: fn(String, i32, String, String) -> String,
}

impl YearRunner {
    pub const fn new(year: i32, runner: fn(String, i32, String, String) -> String) -> Self {
        Self { year, runner }
    }
}

inventory::collect!(YearRunner);

pub fn get_year_runner(year: i32) -> Option<fn(String, i32, String, String) -> String> {
    for runner in inventory::iter::<YearRunner> {
        if runner.year == year {
            return Some(runner.runner);
        }
    }
    None
}
