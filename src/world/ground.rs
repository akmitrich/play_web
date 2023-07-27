use serde_json::Value;

pub async fn section_by_id<'a>(ground: &'a Value, section_id: &str) -> &'a Value {
    crate::util::get(ground, &["sections", section_id])
}

pub async fn points_mut(ground: &mut Value) -> &mut Value {
    crate::util::get_mut(ground, &["points"])
}

pub async fn points_on_section(ground: &Value, section: &Value) -> Value {
    let points = crate::util::get(dbg!(section), &["points"]);
    crate::util::map(dbg!(points), |p| {
        if let Value::String(point_id) = p {
            crate::util::get(ground, &["points", point_id, "label"]).to_owned()
        } else {
            panic!("Point ID must be String.")
        }
    })
}
