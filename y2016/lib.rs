pub mod solutions {
    automod::dir!(pub "./solutions");
}

use solutions::*;

use std::collections::HashMap;
use utils::harness::{RunDay, Solve};

use macros::GetDays;
use proc_macros::GetDays;

#[derive(GetDays)]
pub struct Y2016;

impl RunDay for Y2016 {
    fn fetch_days() -> HashMap<String, fn(i32, String, String) -> String> {
        let days = Y2016::get_days();
        return days;
    }
}
