use crate::util;
use serde_json::Value;

pub async fn points_mut(ground: &mut Value) -> &mut Value {
    crate::util::get_mut(ground, &["points"])
}

pub async fn set_label(points: &mut Value, point_id: &str, label: Value) {
    util::set(points, &[point_id, "label"], validate_label(label));
}

fn validate_label(label: Value) -> Value {
    match label {
        Value::Array(a) => match a.first() {
            Some(label) => validate_label(label.to_owned()),
            None => Value::Null,
        },
        Value::Object(o) => match o.get("label") {
            Some(label) => validate_label(label.to_owned()),
            None => Value::Null,
        },
        _ => label,
    }
}
