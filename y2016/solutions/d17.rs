use md5::compute;

use utils::{file_reader, harness::Solve};

pub struct D17;

impl Solve for D17 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let password = inputs.first().unwrap().clone();

        let mut paths: Vec<String> = vec![];

        walk(&password, 0, 0, &"".to_string(), &mut paths);

        paths.sort_by(|x, y| x.len().cmp(&y.len()));

        let shortest = paths.first().unwrap();

        return shortest.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let password = inputs.first().unwrap().clone();

        let mut paths: Vec<String> = vec![];

        walk(&password, 0, 0, &"".to_string(), &mut paths);

        paths.sort_by(|x, y| x.len().cmp(&y.len()));

        let longest: String = paths
            .iter()
            .rev()
            .collect::<Vec<&String>>()
            .first()
            .unwrap()
            .to_string();

        return longest.len().to_string();
    }
}

fn walk(password: &String, x: i32, y: i32, path: &String, paths: &mut Vec<String>) {
    if x == 3 && y == 3 {
        paths.push(path.clone());
        return;
    }
    let dir_chars = ['U', 'D', 'L', 'R'];
    for (dir, (new_x, new_y)) in [(0, -1), (0, 1), (-1, 0), (1, 0)].iter().enumerate() {
        if is_open(x, y, &password, &path, dir) {
            let mut new_path = path.clone();
            new_path.push(dir_chars[dir]);
            walk(password, x + *new_x, y + *new_y, &new_path, paths);
        }
    }
}

fn is_open(x: i32, y: i32, password: &String, path: &String, dir: usize) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    if x > 3 || y > 3 {
        return false;
    }
    let hash = hash(password, path);
    let hash_chars: Vec<char> = hash.chars().collect();
    let dir_char = hash_chars[dir];
    if ['b', 'c', 'd', 'e', 'f'].contains(&dir_char) {
        return true;
    }
    return false;
}

fn hash(password: &String, path: &String) -> String {
    let message = format!("{}{}", password, path);
    let digest = compute(message.clone());
    let hex_digest = format!("{:x}", digest);
    return hex_digest;
}
