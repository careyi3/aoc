pub mod d01;

use d01::D01;

use std::collections::HashMap;
use utils::harness::{RunDay, Solve};

pub struct Y2016;

impl RunDay for Y2016 {
    fn fetch_days() -> HashMap<i32, fn(i32, String, String) -> String> {
        return HashMap::from([
            (1, D01::solve as fn(i32, String, String) -> String),
        ]);
    }
}
