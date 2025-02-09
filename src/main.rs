use std::collections::HashMap;
use std::env;
use std::process;

use utils::harness::RunDay;

use y2015::Y2015;
use y2016::Y2016;

fn main() {
    let (year, day, part, input) = parse_args();
    let path = format!("./inputs/{}/{}/{}", year, day, input);
    let func = fetch_func(year);
    let answer = func(day, part, input, path);
    println!("Answer:\t{}", answer);
}

fn fetch_func(year: i32) -> fn(i32, i32, String, String) -> String {
    let years = HashMap::from([
        (
            2015,
            Y2015::run_day as fn(i32, i32, String, String) -> String,
        ),
        (
            2016,
            Y2016::run_day as fn(i32, i32, String, String) -> String,
        ),
    ]);
    return *years.get(&year).unwrap();
}

fn parse_args() -> (i32, i32, i32, String) {
    let mut args: Vec<String> = env::args().collect();
    let year_num;
    let day_num;
    let part_num;
    let input;

    if args.len() == 2 {
        args = args[1].split(' ').map(|v| v.to_string()).collect();
    } else {
        args.remove(0);
    }

    match args.len() {
        4 => {
            year_num = match args[0].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: year argument not an integer");
                    help();
                    process::exit(1)
                }
            };
            day_num = match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: day argument not an integer");
                    help();
                    process::exit(1)
                }
            };
            part_num = match args[2].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: part argument not an integer");
                    help();
                    process::exit(1)
                }
            };
            input = match &args[3][..] {
                "sample" => "sample".to_string(),
                "input" => "input".to_string(),
                _ => {
                    eprintln!("error: invalid input type");
                    help();
                    process::exit(1)
                }
            }
        }
        _ => {
            help();
            process::exit(1)
        }
    }

    return (year_num, day_num, part_num, input);
}

fn help() {
    println!(
        "usage:
aoc <year> <day> <part> {{sample|input}}"
    );
}
