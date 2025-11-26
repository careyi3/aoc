pub mod solutions {
    automod::dir!(pub "./solutions");
}

use solutions::*;

use std::collections::HashMap;
use utils::harness::{RunDay, Solve};

use macros::GetDays;
use proc_macros::GetDays;

#[derive(GetDays)]
pub struct Y2017;

impl RunDay for Y2017 {
    fn fetch_days() -> HashMap<String, fn(i32, String, String) -> String> {
        Y2017::get_days()
    }
}

inventory::submit! {
    utils::harness::YearRunner::new(2017, Y2017::run_day)
}
