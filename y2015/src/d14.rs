use utils::{file_reader, harness::Solve};

pub struct D14;

impl Solve for D14 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);

        let mut largest = 0;
        for input in inputs {
            let words: Vec<&str> = input.split(' ').collect();
            let speed: i32 = words[3].parse().unwrap();
            let moving: i32 = words[6].parse().unwrap();
            let resting: i32 = words[13].parse().unwrap();
            let dist = calculate_dist(2503, speed, resting, moving);
            if dist > largest {
                largest = dist;
            }
        }

        return largest.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        let mut scores = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        for t in 1..=2503 {
            let mut dists = [0, 0, 0, 0, 0, 0, 0, 0, 0];
            for (i, input) in inputs.iter().enumerate() {
                let words: Vec<&str> = input.split(' ').collect();
                let speed: i32 = words[3].parse().unwrap();
                let moving: i32 = words[6].parse().unwrap();
                let resting: i32 = words[13].parse().unwrap();
                let dist = calculate_dist(t, speed, resting, moving);
                dists[i] = dist;
            }
            let max_dist = dists.iter().max().unwrap();
            for (i, dist) in dists.iter().enumerate() {
                if dist == max_dist {
                    scores[i] += 1;
                }
            }
        }

        return scores.iter().max().unwrap().to_string();
    }
}

fn calculate_dist(time: i32, speed: i32, resting: i32, moving: i32) -> i32 {
    let total = moving + resting;
    let whole = time / total;
    let remainder = time % total;
    let mut dist = whole * speed * moving;
    if remainder > moving {
        dist += speed * moving;
    } else {
        dist += remainder * speed;
    }
    return dist;
}
