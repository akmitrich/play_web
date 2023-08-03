use crate::util;
use serde_json::Value;

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
    let path = ["current"];
    let delta_t = util::get_num(schedule, &["delta_t"]).unwrap_or(1e-3);
    let next_step = util::get_num(schedule, &path).map(|x| x + delta_t);
    if let Some(next) = next_step {
        util::set_num(schedule, &path, next);
    }
}
