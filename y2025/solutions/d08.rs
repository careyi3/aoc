use anyhow::Result;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use utils::{file_reader, harness::SolveResult};

pub struct D08;

impl SolveResult for D08 {
    fn part1(_input: String, path: &String) -> Result<String> {
        let mut points: Vec<(i64, i64, i64)> = vec![];
        for line in file_reader::lines_iter(path)? {
            let nums: Vec<i64> = line?
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            let point = (nums[0], nums[1], nums[2]);
            points.push(point);
        }

        let ordered: Vec<Vec<&(i64, i64, i64)>> = points
            .iter()
            .combinations(2)
            .sorted_by(|a, b| euc_dist(a, b))
            .collect();

        let mut circuits: Vec<HashSet<(i64, i64, i64)>> = vec![];
        let mut connected_pairs = 0;
        for pair in ordered {
            let a = pair[0];
            let b = pair[1];
            if connected_pairs == 999 {
                break;
            }
            if already_linked(&circuits, a, b) {
                connected_pairs += 1;
                continue;
            } else {
                link_points(&mut circuits, a, b);
                connected_pairs += 1;
            }
        }

        let final_circuits = combine_circuits(circuits);

        let ans = final_circuits[0].len() * final_circuits[1].len() * final_circuits[2].len();

        return Ok(ans.to_string());
    }

    fn part2(_input: String, path: &String) -> Result<String> {
        let mut points: Vec<(i64, i64, i64)> = vec![];
        for line in file_reader::lines_iter(path)? {
            let nums: Vec<i64> = line?
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            let point = (nums[0], nums[1], nums[2]);
            points.push(point);
        }

        let ordered: Vec<Vec<&(i64, i64, i64)>> = points
            .iter()
            .combinations(2)
            .sorted_by(|a, b| euc_dist(a, b))
            .collect();

        let mut circuits: Vec<HashSet<(i64, i64, i64)>> = vec![];
        let mut x_1: i64 = 0;
        let mut x_2: i64 = 0;
        for pair in ordered {
            let a = pair[0];
            let b = pair[1];

            if already_linked(&circuits, a, b) {
                continue;
            } else {
                link_points(&mut circuits, a, b);
            }
            circuits = combine_circuits(circuits);
            if circuits.len() == 1 && circuits[0].len() == 1000 {
                x_1 = a.0;
                x_2 = b.0;
                break;
            }
        }

        return Ok((x_1 * x_2).to_string());
    }
}

fn euc_dist(a: &Vec<&(i64, i64, i64)>, b: &Vec<&(i64, i64, i64)>) -> Ordering {
    let a_x1 = a[0].0;
    let a_y1 = a[0].1;
    let a_z1 = a[0].2;

    let a_x2 = a[1].0;
    let a_y2 = a[1].1;
    let a_z2 = a[1].2;

    let b_x1 = b[0].0;
    let b_y1 = b[0].1;
    let b_z1 = b[0].2;

    let b_x2 = b[1].0;
    let b_y2 = b[1].1;
    let b_z2 = b[1].2;

    let dist_a =
        (((a_x2 - a_x1).pow(2) + (a_y2 - a_y1).pow(2) + (a_z2 - a_z1).pow(2)) as f64).sqrt() as i64;
    let dist_b =
        (((b_x2 - b_x1).pow(2) + (b_y2 - b_y1).pow(2) + (b_z2 - b_z1).pow(2)) as f64).sqrt() as i64;
    return Ord::cmp(&dist_a, &dist_b);
}

fn already_linked(
    circuits: &Vec<HashSet<(i64, i64, i64)>>,
    a: &(i64, i64, i64),
    b: &(i64, i64, i64),
) -> bool {
    for circuit in circuits {
        if circuit.contains(a) && circuit.contains(b) {
            return true;
        }
    }
    return false;
}

fn link_points(
    circuits: &mut Vec<HashSet<(i64, i64, i64)>>,
    a: &(i64, i64, i64),
    b: &(i64, i64, i64),
) {
    let mut new_circuit = true;
    for circuit in &mut *circuits {
        if circuit.contains(a) {
            circuit.insert(*b);
            new_circuit = false;
        }
        if circuit.contains(b) {
            circuit.insert(*a);
            new_circuit = false;
        }
    }
    if new_circuit {
        let mut new_circuit: HashSet<(i64, i64, i64)> = HashSet::new();
        new_circuit.insert(*a);
        new_circuit.insert(*b);
        circuits.push(new_circuit);
    }
}

fn combine_circuits(circuits: Vec<HashSet<(i64, i64, i64)>>) -> Vec<HashSet<(i64, i64, i64)>> {
    if circuits.is_empty() {
        return vec![];
    }

    let mut combined = circuits;
    let mut changed = true;

    while changed {
        changed = false;
        let mut new_combined: Vec<HashSet<(i64, i64, i64)>> = vec![];
        let mut used = vec![false; combined.len()];

        for i in 0..combined.len() {
            if used[i] {
                continue;
            }

            let mut merged = combined[i].clone();
            used[i] = true;

            for j in (i + 1)..combined.len() {
                if used[j] {
                    continue;
                }

                if merged.intersection(&combined[j]).count() > 0 {
                    merged = merged.union(&combined[j]).copied().collect();
                    used[j] = true;
                    changed = true;
                }
            }

            new_combined.push(merged);
        }

        combined = new_combined;
    }

    combined.sort_by(|a, b| b.len().cmp(&a.len()));
    combined
}
