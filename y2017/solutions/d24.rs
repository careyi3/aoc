use utils::{file_reader, harness::Solve};

pub struct D24;

impl Solve for D24 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let components: Vec<(i32, i32)> = inputs
            .iter()
            .map(|x| {
                x.split('/')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|x| (x[0], x[1]))
            .collect();

        let ans = build(components, 0, 0);

        return ans.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let components: Vec<(i32, i32)> = inputs
            .iter()
            .map(|x| {
                x.split('/')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|x| (x[0], x[1]))
            .collect();

        let ans = build_longest(components, 0, 0, 0);

        return ans.0.to_string();
    }
}

fn build_longest(
    components: Vec<(i32, i32)>,
    last_value: i32,
    sum: i32,
    length: i32,
) -> (i32, i32) {
    if components.len() == 0 {
        return (sum, length);
    }
    let mut vals: Vec<(i32, i32)> = vec![];
    for (idx, comp) in components.clone().iter().enumerate() {
        if comp.0 == last_value {
            let mut removed = components.clone();
            removed.remove(idx);
            vals.push(build_longest(
                removed,
                comp.1,
                sum + comp.1 + comp.0,
                length + 1,
            ));
        }
        if comp.1 == last_value {
            let mut removed = components.clone();
            removed.remove(idx);
            vals.push(build_longest(
                removed,
                comp.0,
                sum + comp.1 + comp.0,
                length + 1,
            ));
        }
    }
    if vals.len() == 0 {
        return (sum, length);
    }
    vals.sort_by_key(|&(s, l)| (l, s));
    return vals[vals.len() - 1];
}

fn build(components: Vec<(i32, i32)>, last_value: i32, sum: i32) -> i32 {
    if components.len() == 0 {
        return sum;
    }
    let mut vals: Vec<i32> = vec![];
    for (idx, comp) in components.clone().iter().enumerate() {
        if comp.0 == last_value {
            let mut removed = components.clone();
            removed.remove(idx);
            vals.push(build(removed, comp.1, sum + comp.1 + comp.0));
        }
        if comp.1 == last_value {
            let mut removed = components.clone();
            removed.remove(idx);
            vals.push(build(removed, comp.0, sum + comp.1 + comp.0));
        }
    }
    if vals.len() == 0 {
        return sum;
    }
    return *vals.iter().max().unwrap();
}
