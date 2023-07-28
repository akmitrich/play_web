use serde_json::Value;

pub async fn points_mut(ground: &mut Value) -> &mut Value {
    crate::util::get_mut(ground, &["points"])
}
