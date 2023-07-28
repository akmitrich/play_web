use crate::util;
use serde_json::Value;

pub fn prepare_to_move(rolling: &mut Value) {
    let _ = util::get_mut(rolling, &["cars"]);
}

pub fn do_move(rolling: &mut Value) {
    let _ = util::get_mut(rolling, &["cars"]);
}

pub fn after_move(rolling: &mut Value) {
    let _ = util::get_mut(rolling, &["cars"]);
}
