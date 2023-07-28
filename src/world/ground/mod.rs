use crate::util;
use serde_json::Value;

mod point;
mod section;

pub use point::{points_mut, set_label};
pub use section::section_by_id;

pub fn prepare_to_move(ground: &mut Value) {
    let _ = util::get_mut(ground, &["points"]);
}

pub fn do_move(ground: &mut Value) {
    let _ = util::get_mut(ground, &["points"]);
}

pub fn after_move(ground: &mut Value) {
    let _ = util::get_mut(ground, &["points"]);
}

pub async fn points_on_section(ground: &Value, section: &Value) -> Value {
    let points = util::get(dbg!(section), &["points"]);
    util::map(points, |p| {
        if let Value::String(point_id) = p {
            util::get(ground, &["points", point_id, "label"]).to_owned()
        } else {
            panic!("Point ID must be String.")
        }
    })
}
