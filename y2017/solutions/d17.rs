use utils::{file_reader, harness::Solve};

pub struct D17;

impl Solve for D17 {
    fn part1(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let loop_count: usize = input.first().unwrap().parse().unwrap();
        let mut spin_lock: Vec<usize> = vec![0];
        let mut i: usize = 0;
        let mut position: usize = 0;
        while spin_lock.len() < 2018 {
            let spin_lock_length = spin_lock.len();
            let offset = loop_count % spin_lock_length;
            position = ((position + offset) + 1) % spin_lock_length;
            i += 1;
            spin_lock.insert(position, i);
        }

        return spin_lock[position + 1].to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let loop_count: usize = input.first().unwrap().parse().unwrap();
        let mut position: usize = 0;
        let mut length = 1;
        let mut zero_position = 0;
        while length < 12_000_000 {
            let offset = loop_count % length;
            position = ((position + offset) + 1) % length;
            if position == 0 {
                zero_position = length;
            }
            length += 1;
        }

        return zero_position.to_string();
    }
}
