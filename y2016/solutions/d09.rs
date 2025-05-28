use std::collections::HashMap;

use fancy_regex::Regex;
use utils::{file_reader, harness::Solve};

pub struct D09;

impl Solve for D09 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        let markers = get_markers(&input[0]);

        let mut output = 0;
        let mut i: usize = 0;
        let input_len = input[0].len();

        while i < input_len {
            if markers.contains_key(&i) {
                let (end, length, n) = markers[&i];

                output += length * n;

                i = end + length;
            } else {
                output += 1;
                i += 1;
            }
        }

        return output.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);

        let output = decompress(&input[0]);

        return output.to_string();
    }
}

fn get_markers(input: &String) -> HashMap<usize, (usize, usize, usize)> {
    let rg = Regex::new(r"(\((\d+)x(\d+)\))").unwrap();
    let mut markers: HashMap<usize, (usize, usize, usize)> = HashMap::new();

    for cap in rg.captures_iter(input) {
        let capture = cap.unwrap();
        let idx = capture.get(0).unwrap().start();
        let end = capture.get(0).unwrap().end();
        let length: usize = capture.get(2).unwrap().as_str().parse().unwrap();
        let n: usize = capture.get(3).unwrap().as_str().parse().unwrap();

        markers.insert(idx, (end, length, n));
    }
    return markers;
}

fn decompress(input: &String) -> usize {
    let markers = get_markers(input);

    let mut output = 0;
    let mut i: usize = 0;
    let input_len = input.len();

    while i < input_len {
        if markers.contains_key(&i) {
            let (end, length, n) = markers[&i];

            let inner = decompress(&input[end..end + length].to_string());

            output += n * inner;

            i = end + length;
        } else {
            output += 1;
            i += 1;
        }
    }

    return output;
}
