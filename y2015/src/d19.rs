use fancy_regex::Regex;
use std::collections::{HashMap, HashSet};
use utils::{file_reader, harness::Solve};

pub struct D19;

impl Solve for D19 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut transitions = true;
        let mut rules: HashMap<String, Vec<String>> = HashMap::new();
        let mut molocule = "".to_string();
        let molocules: HashSet<String> = HashSet::new();
        for input in inputs {
            if input == "".to_string() {
                transitions = false;
            }
            if transitions {
                let tuple: Vec<String> = input.split(" => ").map(|x| x.to_string()).collect();
                rules
                    .entry(tuple[0].clone())
                    .and_modify(|f| f.push(tuple[1].clone()))
                    .or_insert(vec![tuple[1].clone()]);
            } else {
                molocule = input;
            }
        }

        for key in rules.keys() {
            let re = Regex::new(key).unwrap();

            let num = re.find_iter(&molocule).count();
            for _n in 0..num {}
        }

        return molocules.len().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        return input.first().unwrap().to_string();
    }
}
