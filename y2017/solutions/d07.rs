use std::collections::HashMap;

use utils::{file_reader, harness::Solve};

pub struct D07;

impl Solve for D07 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut tree: HashMap<String, String> = HashMap::new();
        for input in inputs {
            let segments: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
            let parent = segments[0].to_string();
            if segments.len() == 2 {
                tree.entry(parent.clone()).or_insert("".to_string());
            } else {
                tree.entry(parent.clone()).or_insert("".to_string());
                for i in 3..segments.len() {
                    let val = &segments[i];
                    let node = val.replace(",", "");
                    tree.entry(node)
                        .and_modify(|x| *x = parent.to_string())
                        .or_insert(parent.to_string());
                }
            }
        }

        let mut root: String = "".to_string();
        for (key, val) in tree {
            if val == "".to_string() {
                root = key;
                break;
            }
        }

        return root;
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut nodes: HashMap<String, (String, i32)> = HashMap::new();
        for input in inputs {
            let segments: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
            let parent = segments[0].to_string();
            let weight: i32 = segments[1]
                .replace("(", "")
                .replace(")", "")
                .parse()
                .unwrap();
            if segments.len() == 2 {
                nodes
                    .entry(parent.clone())
                    .and_modify(|x| *x = (x.0.clone(), weight))
                    .or_insert(("".to_string(), weight));
            } else {
                nodes
                    .entry(parent.clone())
                    .and_modify(|x| *x = (x.0.clone(), weight))
                    .or_insert(("".to_string(), weight));
                for i in 3..segments.len() {
                    let val = &segments[i];
                    let node = val.replace(",", "");
                    nodes
                        .entry(node)
                        .and_modify(|x| *x = (parent.to_string(), x.1))
                        .or_insert((parent.to_string(), 0));
                }
            }
        }

        let mut tree: HashMap<String, Vec<(String, i32)>> = HashMap::new();
        for (node, (parent, weight)) in nodes {
            tree.entry(parent)
                .and_modify(|x| x.push((node.clone(), weight)))
                .or_insert(vec![(node.clone(), weight)]);
        }

        let root = &tree[&"".to_string()].first().unwrap().0;

        let mut ans: Vec<i32> = vec![];
        walk(&tree, root.clone(), &mut ans);

        return ans.first().unwrap().to_string();
    }
}

fn walk(tree: &HashMap<String, Vec<(String, i32)>>, node: String, ans: &mut Vec<i32>) -> i32 {
    let mut weights: HashMap<i32, Vec<i32>> = HashMap::new();
    if !&tree.contains_key(&node) {
        return 0;
    }
    let mut sum = 0;
    for (node, weight) in &tree[&node] {
        let mut total_weight = *weight;
        total_weight += walk(tree, node.clone(), ans);
        sum += total_weight;
        weights
            .entry(total_weight)
            .and_modify(|x| x.push(*weight))
            .or_insert(vec![*weight]);
    }

    if weights.len() == 1 {
        return sum;
    } else {
        let mut to_change = 0;
        let mut common_weight = 0;
        let mut outlier_weight = 0;
        for (total_weight, node_weights) in weights.clone() {
            if node_weights.len() == 1 {
                outlier_weight = total_weight;
                to_change = *node_weights.first().unwrap();
            } else {
                common_weight = total_weight;
            }
        }

        if outlier_weight > common_weight {
            ans.push(to_change - (outlier_weight - common_weight));
        } else {
            ans.push(to_change + (common_weight - outlier_weight));
        }

        return sum;
    }
}
