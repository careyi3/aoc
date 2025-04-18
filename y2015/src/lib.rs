pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;
pub mod d09;
pub mod d10;
pub mod d11;
pub mod d12;
pub mod d13;
pub mod d14;
pub mod d15;
pub mod d16;
pub mod d17;
pub mod d18;
pub mod d19;
pub mod d20;
pub mod d21;
pub mod d22;

use d01::D01;
use d02::D02;
use d03::D03;
use d04::D04;
use d05::D05;
use d06::D06;
use d07::D07;
use d08::D08;
use d09::D09;
use d10::D10;
use d11::D11;
use d12::D12;
use d13::D13;
use d14::D14;
use d15::D15;
use d16::D16;
use d17::D17;
use d18::D18;
use d19::D19;
use d20::D20;
use d21::D21;
use d22::D22;

use std::collections::HashMap;
use utils::harness::{RunDay, Solve};

pub struct Y2015;

impl RunDay for Y2015 {
    fn fetch_days() -> HashMap<i32, fn(i32, String, String) -> String> {
        return HashMap::from([
            (1, D01::solve as fn(i32, String, String) -> String),
            (2, D02::solve as fn(i32, String, String) -> String),
            (3, D03::solve as fn(i32, String, String) -> String),
            (4, D04::solve as fn(i32, String, String) -> String),
            (5, D05::solve as fn(i32, String, String) -> String),
            (6, D06::solve as fn(i32, String, String) -> String),
            (7, D07::solve as fn(i32, String, String) -> String),
            (8, D08::solve as fn(i32, String, String) -> String),
            (9, D09::solve as fn(i32, String, String) -> String),
            (10, D10::solve as fn(i32, String, String) -> String),
            (11, D11::solve as fn(i32, String, String) -> String),
            (12, D12::solve as fn(i32, String, String) -> String),
            (13, D13::solve as fn(i32, String, String) -> String),
            (14, D14::solve as fn(i32, String, String) -> String),
            (15, D15::solve as fn(i32, String, String) -> String),
            (16, D16::solve as fn(i32, String, String) -> String),
            (17, D17::solve as fn(i32, String, String) -> String),
            (18, D18::solve as fn(i32, String, String) -> String),
            (19, D19::solve as fn(i32, String, String) -> String),
            (20, D20::solve as fn(i32, String, String) -> String),
            (21, D21::solve as fn(i32, String, String) -> String),
            (22, D22::solve as fn(i32, String, String) -> String),
        ]);
    }
}
