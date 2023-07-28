use serde_json::Value;

mod point;
mod section;

pub use point::{points_mut, set_label};
pub use section::section_by_id;

pub async fn points_on_section(ground: &Value, section: &Value) -> Value {
    let points = crate::util::get(dbg!(section), &["points"]);
    crate::util::map(points, |p| {
        if let Value::String(point_id) = p {
            crate::util::get(ground, &["points", point_id, "label"]).to_owned()
        } else {
            panic!("Point ID must be String.")
        }
    })
}