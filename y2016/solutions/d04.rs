use std::collections::HashMap;
use utils::{file_reader, harness::Solve};

pub struct D04;

impl Solve for D04 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut sum = 0;
        for input in inputs {
            let mut segments: Vec<String> = input.split('-').map(|s| s.to_string()).collect();
            let mut sec_che: Vec<String> = segments
                .pop()
                .unwrap()
                .split('[')
                .map(|s| s.to_string())
                .collect();
            let sector_id: i32 = sec_che[0].parse().unwrap();
            let checksum = &mut sec_che[1];
            checksum.pop();

            let mut map: HashMap<char, i32> = HashMap::new();
            for segment in segments {
                for char in segment.chars() {
                    map.entry(char).and_modify(|x| *x += 1).or_insert(1);
                }
            }

            let mut top: Vec<(char, i32)> = map.iter().map(|x| (*x.0, *x.1)).collect();
            top.sort_unstable_by_key(|x| (x.1, 200 - x.0 as i32));
            top.reverse();

            let new_checksum: Vec<char> = top.into_iter().map(|x| x.0).collect();
            let value: String = new_checksum.into_iter().collect();

            if value[..5] == *checksum {
                sum += sector_id;
            }
        }

        return sum.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut valids: Vec<(Vec<String>, i32)> = vec![];
        for input in inputs {
            let mut segments: Vec<String> = input.split('-').map(|s| s.to_string()).collect();
            let mut seg_copy = segments.clone();
            seg_copy.pop();
            let mut sec_che: Vec<String> = segments
                .pop()
                .unwrap()
                .split('[')
                .map(|s| s.to_string())
                .collect();
            let sector_id: i32 = sec_che[0].parse().unwrap();
            let checksum = &mut sec_che[1];
            checksum.pop();

            let mut map: HashMap<char, i32> = HashMap::new();
            for segment in segments {
                for char in segment.chars() {
                    map.entry(char).and_modify(|x| *x += 1).or_insert(1);
                }
            }

            let mut top: Vec<(char, i32)> = map.iter().map(|x| (*x.0, *x.1)).collect();
            top.sort_unstable_by_key(|x| (x.1, 200 - x.0 as i32));
            top.reverse();

            let new_checksum: Vec<char> = top.into_iter().map(|x| x.0).collect();
            let value: String = new_checksum.into_iter().collect();

            if value[..5] == *checksum {
                valids.push((seg_copy, sector_id));
            }
        }

        let mut answers: HashMap<String, i32> = HashMap::new();
        for (segments, secord_id) in valids {
            let mut words: Vec<String> = vec![];
            for segment in segments {
                let mut word = "".to_string();
                let start: u8 = 97;
                for char in segment.chars() {
                    let char_int = char as u8;
                    let idx = char_int - 97;
                    let offset = (secord_id % 26) as u8;
                    let new_char_idx = (idx + offset) % 26;
                    word.push((new_char_idx + start) as char);
                }
                words.push(word);
            }
            answers.insert(words.join(" "), secord_id);
        }

        return answers["northpole object storage"].to_string();
    }
}
