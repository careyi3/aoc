use fancy_regex::Regex;
use serde_json::{self, Value};
use utils::{file_reader, harness::Solve};

pub struct D12;

impl Solve for D12 {
    fn part1(_input: String, path: &String) -> String {
        let lines = file_reader::read_lines(path);
        let input = lines.first().unwrap();

        let matches = Regex::new(r"-?[0-9]{0,10}").unwrap();

        let mut ans = 0;
        for m in matches.find_iter(input) {
            let val = m.unwrap().as_str();
            match val.parse::<i32>() {
                Ok(n) => {
                    ans += n;
                }
                Err(_) => {}
            };
        }

        return ans.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let lines = file_reader::read_lines(path);
        let input = lines.first().unwrap();
        let data: Value = serde_json::from_str(input).unwrap();

        let mut ans = 0;
        walk(&data, &mut ans);

        return ans.to_string();
    }
}

fn walk(data: &Value, ans: &mut i64) {
    match data {
        Value::Object(n) => {
            if n.values().any(|x| x.is_string() && x == "red") {
                return;
            } else {
                for i in n.values() {
                    walk(i, ans);
                }
            }
        }
        Value::Array(n) => {
            for i in n {
                walk(i, ans);
            }
        }
        Value::Number(n) => {
            *ans += n.as_i64().unwrap();
        }
        _ => {}
    }
}
