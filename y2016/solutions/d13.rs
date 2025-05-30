use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use utils::harness::Solve;

pub struct D13;

impl Solve for D13 {
    fn part1(_input: String, _path: &String) -> String {
        let mut points: HashMap<(i32, i32), i32> = HashMap::new();

        let mut pq = seed_pq();

        let x = 1;
        let y = 1;
        while x != 31 && y != 39 {
            if pq.is_empty() {
                break;
            }
            let ((x, y), dist) = pq.pop().unwrap();
            let dist_num = dist.0;
            points.insert((x, y), dist.0);
            for dir in [[1, 0], [-1, 0], [0, 1], [0, -1]] {
                let next_x = x + dir[0];
                let next_y = y + dir[1];
                let item = pq.get(&(next_x, next_y));
                match item {
                    Some(item) => {
                        if dist_num + 1 < item.1 .0 {
                            pq.change_priority(&(next_x, next_y), Reverse(dist_num + 1));
                        }
                    }
                    None => {}
                }
            }
        }

        return points[&(31, 39)].to_string();
    }

    fn part2(_input: String, _path: &String) -> String {
        let mut points: HashMap<(i32, i32), i32> = HashMap::new();

        let mut pq = seed_pq();

        let x = 1;
        let y = 1;
        while x != 31 && y != 39 {
            if pq.is_empty() {
                break;
            }
            let ((x, y), dist) = pq.pop().unwrap();
            let dist_num = dist.0;
            points.insert((x, y), dist.0);
            for dir in [[1, 0], [-1, 0], [0, 1], [0, -1]] {
                let next_x = x + dir[0];
                let next_y = y + dir[1];
                let item = pq.get(&(next_x, next_y));
                match item {
                    Some(item) => {
                        if dist_num + 1 < item.1 .0 {
                            pq.change_priority(&(next_x, next_y), Reverse(dist_num + 1));
                        }
                    }
                    None => {}
                }
            }
        }

        let mut count = 0;
        for (_, dist) in points {
            if dist <= 50 {
                count += 1;
            }
        }

        return count.to_string();
    }
}

fn seed_pq() -> PriorityQueue<(i32, i32), Reverse<i32>> {
    let mut pq: PriorityQueue<(i32, i32), Reverse<i32>> = PriorityQueue::new();

    for x in 0..100 {
        for y in 0..100 {
            if is_open(x, y) {
                if x == 1 && y == 1 {
                    pq.push((x, y), Reverse(0));
                } else {
                    pq.push((x, y), Reverse(1000000));
                }
            }
        }
    }

    return pq;
}

fn is_open(x: i32, y: i32) -> bool {
    if x < 0 {
        return false;
    }
    if y < 0 {
        return false;
    }
    let val = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + 1364;
    let bits = format!("{val:b}");

    let mut ones = 0;
    for c in bits.chars() {
        if c == '1' {
            ones += 1;
        }
    }

    return ones % 2 == 0;
}
