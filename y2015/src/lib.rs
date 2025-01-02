pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;

use d01::D01;
use d02::D02;
use d03::D03;
use d04::D04;

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
        ]);
    }
}
