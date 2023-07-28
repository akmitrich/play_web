use crate::util;
use serde_json::Value;

pub fn get_start(schedule: &Value) -> Option<chrono::DateTime<chrono::Local>> {
    let start = util::get(schedule, &["start"]);
    serde_json::from_value(start.to_owned()).ok()
}

pub fn get_current(schedule: &Value) -> Option<chrono::DateTime<chrono::Local>> {
    let start = get_start(schedule)?;
    let seconds = serde_json::from_value(util::get(schedule, &["current"]).to_owned()).unwrap_or(0);
    Some(start + chrono::Duration::seconds(seconds))
}
