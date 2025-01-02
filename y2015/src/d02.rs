use utils::file_reader::read_lines;
use utils::harness::Solve;

pub struct D02;

impl Solve for D02 {
    fn part1(_input: String, path: &String) -> String {
        let mut sum = 0;
        for line in read_lines(path) {
            let v: Vec<i32> = line.split('x').map(|x| x.parse().unwrap()).collect();
            let (x, y, z) = (v[0], v[1], v[2]);
            let sides = vec![x * y, y * z, x * z];
            sum += (2 * sides[0]) + (2 * sides[1]) + (2 * sides[2]);
            sum += sides.iter().min().unwrap();
        }
        return sum.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let mut sum = 0;
        for line in read_lines(path) {
            let v: Vec<i32> = line.split('x').map(|x| x.parse().unwrap()).collect();
            let (x, y, z) = (v[0], v[1], v[2]);
            let faces = vec![(2 * x) + (2 * y), (2 * y) + (2 * z), (2 * x) + (2 * z)];
            sum += faces.iter().min().unwrap();
            sum += x * y * z;
        }
        return sum.to_string();
    }
}
