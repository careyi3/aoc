use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D06;

impl Solve for D06 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let length = inputs.clone().first().unwrap().chars().into_iter().count();

        let mut data: HashMap<usize, HashMap<char, i32>> = HashMap::new();
        for input in inputs {
            for (i, word) in input.chars().enumerate() {
                data.entry(i)
                    .and_modify(|f| {
                        f.entry(word).and_modify(|s| *s += 1).or_insert(1);
                    })
                    .or_insert(HashMap::from([(word, 1)]));
            }
        }

        let mut ans: String = "".to_string();
        for i in 0 as usize..length {
            let letters = &data[&i];
            let mut contents: Vec<(&char, &i32)> = letters.iter().collect();
            contents.sort_unstable_by_key(|s| (s.1));
            contents.reverse();
            ans.push(*contents.first().unwrap().0);
        }

        return ans;
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let length = inputs.clone().first().unwrap().chars().into_iter().count();

        let mut data: HashMap<usize, HashMap<char, i32>> = HashMap::new();
        for input in inputs {
            for (i, word) in input.chars().enumerate() {
                data.entry(i)
                    .and_modify(|f| {
                        f.entry(word).and_modify(|s| *s += 1).or_insert(1);
                    })
                    .or_insert(HashMap::from([(word, 1)]));
            }
        }

        let mut ans: String = "".to_string();
        for i in 0 as usize..length {
            let letters = &data[&i];
            let mut contents: Vec<(&char, &i32)> = letters.iter().collect();
            contents.sort_unstable_by_key(|s| (s.1));
            ans.push(*contents.first().unwrap().0);
        }

        return ans;
    }
}
