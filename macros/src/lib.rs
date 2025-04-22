use std::collections::HashMap;

pub trait GetDays {
    fn get_days() -> HashMap<String, fn(i32, String, String) -> String>;
}
