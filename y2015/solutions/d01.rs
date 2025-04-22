use utils::file_reader::read_lines;
use utils::harness::Solve;

pub struct D01;

impl Solve for D01 {
    fn part1(_input: String, path: &String) -> String {
        let mut floor = 0;
        for char in read_lines(path).first().unwrap().chars() {
            if char == ')' {
                floor -= 1;
            } else {
                floor += 1;
            }
        }
        return floor.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let mut floor = 0;
        let mut basement_floor = 1;
        for (i, char) in read_lines(path).first().unwrap().chars().enumerate() {
            if char == ')' {
                floor -= 1;
            } else {
                floor += 1;
            }
            if floor == -1 {
                basement_floor += i;
                break;
            }
        }
        return basement_floor.to_string();
    }
}
