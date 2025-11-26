use std::env;
use std::process;

use utils::harness::get_year_runner;

#[allow(unused_imports)]
use y2015::Y2015;
#[allow(unused_imports)]
use y2016::Y2016;
#[allow(unused_imports)]
use y2017::Y2017;
#[allow(unused_imports)]
use y2018::Y2018;

fn main() {
    let (year, day, part, input) = parse_args();
    let path = format!("./inputs/{}/{}/{}", year, day, input);

    let func = get_year_runner(year).unwrap_or_else(|| {
        eprintln!("error: year {} not found", year);
        process::exit(1);
    });

    let answer = func(day, part, input, path);
    println!("Answer:\t{}", answer);
}

fn parse_args() -> (i32, String, i32, String) {
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
