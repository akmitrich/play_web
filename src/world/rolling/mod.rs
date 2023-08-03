use crate::util;
use serde_json::Value;

pub mod car;

pub fn prepare_to_move(rolling: &mut Value) {
    let cars = util::get_mut(rolling, &["cars"]);
    car::clear_acc(cars);
    car::calc_acc(cars);
}

pub fn do_move(rolling: &mut Value, delta_t: f64) {
    let cars = util::get_mut(rolling, &["cars"]);
    car::calc_vel(cars, delta_t);
    car::calc_pos(cars, delta_t);
}

pub fn after_move(rolling: &mut Value) {
    let cars = util::get_mut(rolling, &["cars"]);
    util::iterate_mut(cars, |car| log::trace!("Step completed: {:?}", car));
}
