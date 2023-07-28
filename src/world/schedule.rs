use crate::util;
use serde_json::{json, Value};

pub fn get_start(schedule: &Value) -> Option<chrono::DateTime<chrono::Local>> {
    let start = util::get(schedule, &["start"]);
    serde_json::from_value(start.to_owned()).ok()
}

pub fn get_current(schedule: &Value) -> Option<chrono::DateTime<chrono::Local>> {
    let start = get_start(schedule)?;
    let current = util::get(schedule, &["current"]);
    let seconds = serde_json::from_value(current.to_owned()).unwrap_or(0);
    Some(start + chrono::Duration::seconds(seconds))
}

pub fn advance_by_one(schedule: &mut Value) {
    let current = util::get_mut(schedule, &["current"]);
    if let Value::Number(num) = current {
        *current = json!(num.as_u64().unwrap() + 1);
    }
}
