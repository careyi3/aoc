use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D16;

impl Solve for D16 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let properties: HashMap<String, i32> = HashMap::from([
            ("children".to_string(), 3),
            ("cats".to_string(), 7),
            ("samoyeds".to_string(), 2),
            ("pomeranians".to_string(), 3),
            ("akitas".to_string(), 0),
            ("vizslas".to_string(), 0),
            ("goldfish".to_string(), 5),
            ("trees".to_string(), 3),
            ("cars".to_string(), 2),
            ("perfumes".to_string(), 1),
        ]);

        let mut aunts: Vec<i32> = vec![];
        'aunts: for (i, input) in inputs.iter().enumerate() {
            let props: Vec<String> = input.split(", ").map(|a| a.to_string()).collect();
            for prop in props {
                let values: Vec<String> = prop.split(": ").map(|a| a.to_string()).collect();
                let key = &values[0];
                let val: i32 = values[1].parse().unwrap();

                if properties.contains_key(key) {
                    if properties[key] != val {
                        continue 'aunts;
                    }
                }
            }
            aunts.push(i as i32);
        }

        return (aunts[0] + 1).to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let properties: HashMap<String, i32> = HashMap::from([
            ("children".to_string(), 3),
            ("cats".to_string(), 7),
            ("samoyeds".to_string(), 2),
            ("pomeranians".to_string(), 3),
            ("akitas".to_string(), 0),
            ("vizslas".to_string(), 0),
            ("goldfish".to_string(), 5),
            ("trees".to_string(), 3),
            ("cars".to_string(), 2),
            ("perfumes".to_string(), 1),
        ]);

        let mut aunts: Vec<i32> = vec![];
        'aunts: for (i, input) in inputs.iter().enumerate() {
            let props: Vec<String> = input.split(", ").map(|a| a.to_string()).collect();
            for prop in props {
                let values: Vec<String> = prop.split(": ").map(|a| a.to_string()).collect();
                let key = &values[0];
                let val: i32 = values[1].parse().unwrap();

                if properties.contains_key(key) {
                    if key == "cats" || key == "trees" {
                        if properties[key] >= val {
                            continue 'aunts;
                        }
                    } else if key == "pomeranians" || key == "goldfish" {
                        if properties[key] <= val {
                            continue 'aunts;
                        }
                    } else if properties[key] != val {
                        continue 'aunts;
                    }
                }
            }
            aunts.push(i as i32);
        }

        return (aunts[0] + 1).to_string();
    }
}
