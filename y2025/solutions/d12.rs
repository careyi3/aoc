use anyhow::Result;
use utils::{file_reader, harness::SolveResult};

pub struct D12;

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    values: Vec<usize>,
}

#[derive(Debug)]
struct Input {
    grids: Vec<Grid>,
}

fn parse_input(lines: &[String]) -> Input {
    let mut grids = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            let label = parts[0].trim();

            if label.contains('x') {
                let dims: Vec<usize> = label.split('x').map(|s| s.parse().unwrap()).collect();
                let values: Vec<usize> = parts[1]
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                grids.push(Grid {
                    width: dims[0],
                    height: dims[1],
                    values,
                });
            }
        }
        i += 1;
    }

    Input { grids }
}

impl SolveResult for D12 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let lines = file_reader::read_lines(path);
        let input = parse_input(&lines);

        let mut defo_true = 0;
        for grid in input.grids {
            let mut present_area = 0;
            for value in grid.values {
                present_area += value * 8;
            }
            if present_area < grid.height * grid.width {
                defo_true += 1;
            }
        }

        Ok(defo_true.to_string())
    }

    fn part2(_input: String, _path: &String) -> Result<String> {
        Ok("".to_string())
    }
}
