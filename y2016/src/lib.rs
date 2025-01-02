use std::collections::HashMap;
use utils::harness::RunDay;

pub struct Y2016;

impl RunDay for Y2016 {
    fn fetch_days() -> HashMap<i32, fn(i32, String, String) -> String> {
        return HashMap::from([]);
    }
}
