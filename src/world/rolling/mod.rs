use crate::util;
use serde_json::Value;

pub mod car;

pub fn prepare_to_move(rolling: &mut Value) {
    let cars = util::get_mut(rolling, &["cars"]);
    util::iterate_mut(cars, |car| println!("Prepare: {:?}", car))
}

pub fn do_move(rolling: &mut Value) {
    let cars = util::get_mut(rolling, &["cars"]);
}

pub fn after_move(rolling: &mut Value) {
    let cars = util::get_mut(rolling, &["cars"]);
}
