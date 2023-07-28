use serde_json::Value;

pub async fn section_by_id<'a>(ground: &'a Value, section_id: &str) -> &'a Value {
    crate::util::get(ground, &["sections", section_id])
}
