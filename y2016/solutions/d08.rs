use utils::{file_reader, harness::Solve};

pub struct D08;

impl Solve for D08 {
    fn part1(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        const WIDTH: usize = 50;
        const HEIGHT: usize = 6;
        let mut screen: [[char; WIDTH]; HEIGHT] = [['.'; WIDTH]; HEIGHT];

        for input in inputs {
            let parts: Vec<String> = input.split(' ').map(|x| x.to_string()).collect();
            if parts[0] == "rect".to_string() {
                let ab: Vec<usize> = parts[1].split('x').map(|x| x.parse().unwrap()).collect();
                fill(ab[0], ab[1], &mut screen);
            } else {
                if parts[1] == "column".to_string() {
                    let x: usize = parts[2]
                        .split('=')
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()[1]
                        .parse()
                        .unwrap();
                    let n: usize = parts[4].parse().unwrap();
                    rotate_column(x, n, &mut screen);
                } else {
                    let y: usize = parts[2]
                        .split('=')
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()[1]
                        .parse()
                        .unwrap();
                    let n: usize = parts[4].parse().unwrap();
                    rotate_row(y, n, &mut screen);
                }
            }
        }

        let mut count = 0;
        for line in screen {
            for c in line {
                if c == '#' {
                    count += 1;
                }
            }
        }

        return count.to_string();
    }

    fn part2(_input: String, path: &String) -> String {
        let inputs = file_reader::read_lines(path);
        const WIDTH: usize = 50;
        const HEIGHT: usize = 6;
        let mut screen: [[char; WIDTH]; HEIGHT] = [['.'; WIDTH]; HEIGHT];

        for input in inputs {
            let parts: Vec<String> = input.split(' ').map(|x| x.to_string()).collect();
            if parts[0] == "rect".to_string() {
                let ab: Vec<usize> = parts[1].split('x').map(|x| x.parse().unwrap()).collect();
                fill(ab[0], ab[1], &mut screen);
            } else {
                if parts[1] == "column".to_string() {
                    let x: usize = parts[2]
                        .split('=')
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()[1]
                        .parse()
                        .unwrap();
                    let n: usize = parts[4].parse().unwrap();
                    rotate_column(x, n, &mut screen);
                } else {
                    let y: usize = parts[2]
                        .split('=')
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()[1]
                        .parse()
                        .unwrap();
                    let n: usize = parts[4].parse().unwrap();
                    rotate_row(y, n, &mut screen);
                }
            }
        }

        print(screen);

        return "".to_string();
    }
}

fn rotate_row(y: usize, n: usize, screen: &mut [[char; 50]; 6]) {
    screen[y].rotate_right(n);
}

fn rotate_column(x: usize, n: usize, screen: &mut [[char; 50]; 6]) {
    let mut temp: [char; 6] = ['.'; 6];
    for (i, line) in screen.iter().enumerate() {
        temp[i] = line[x];
    }
    temp.rotate_right(n);
    for (i, line) in screen.iter_mut().enumerate() {
        line[x] = temp[i];
    }
}

fn fill(a: usize, b: usize, screen: &mut [[char; 50]; 6]) {
    for x in 0..a {
        for y in 0..b {
            screen[y][x] = '#';
        }
    }
}

fn print(screen: [[char; 50]; 6]) {
    for line in screen {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
}
