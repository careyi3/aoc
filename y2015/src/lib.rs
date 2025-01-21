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
        ]);
    }
}
