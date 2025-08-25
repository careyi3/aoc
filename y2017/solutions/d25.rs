use std::collections::{HashMap, HashSet};
use utils::{file_reader, harness::Solve};

pub struct D25;

impl Solve for D25 {
    fn part1(_input: String, _path: &String) -> String {
        let mut state: String = "A".to_string();
        let states: HashMap<String, (i32, i32, String, i32, i32, String)> = HashMap::from([
            (
                "A".to_string(),
                (1, 1, "B".to_string(), 0, 1, "C".to_string()),
            ),
            (
                "B".to_string(),
                (0, -1, "A".to_string(), 0, 1, "D".to_string()),
            ),
            (
                "C".to_string(),
                (1, 1, "D".to_string(), 1, 1, "A".to_string()),
            ),
            (
                "D".to_string(),
                (1, -1, "E".to_string(), 0, -1, "D".to_string()),
            ),
            (
                "E".to_string(),
                (1, 1, "F".to_string(), 1, -1, "B".to_string()),
            ),
            (
                "F".to_string(),
                (1, 1, "A".to_string(), 1, 1, "E".to_string()),
            ),
        ]);
        let step: i32 = 12368930;

        let mut tape: HashSet<i32> = HashSet::new();
        let mut id: i32 = 0;
        for _ in 0..step {
            let s = &states[&state];
            if tape.contains(&id) {
                if s.3 == 0 {
                    tape.remove(&id);
                }
                id += s.4;
                state = s.5.clone();
            } else {
                if s.0 == 1 {
                    tape.insert(id);
                }
                id += s.1;
                state = s.2.clone();
            }
        }

        return tape.len().to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        return input.first().unwrap().to_string();
    }
}
