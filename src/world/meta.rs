use crate::util;
use serde_json::{json, Value};

pub fn get_leader_token(meta: &Value) -> Option<uuid::Uuid> {
    let str_uuid =
        serde_json::from_value::<String>(util::get(meta, &["leader_token"]).to_owned()).ok()?;
    uuid::Uuid::parse_str(&str_uuid).ok()
}

pub fn set_leader_token(meta: &mut Value, token: uuid::Uuid) {
    util::set(meta, &["leader_token"], json!(token.to_string()));
}
