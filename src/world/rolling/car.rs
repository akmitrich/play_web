use crate::util;
use serde_json::{json, Map, Value};

const ZERO: f64 = 0 as _;
const ALPHA: f64 = 0.25;
const GRAV: f64 = 10.;
const L0: f64 = 25_f64;
const K: f64 = 10000_f64;

pub fn clear_acc(cars: &mut Value) {
    util::iterate_mut(cars, |car| util::set(car, &["acc"], json!(0)))
}

pub fn calc_acc(cars: &mut Value) {
    do_coupler(cars);
    calc_pull(cars);
    calc_friction(cars);
}

pub fn calc_vel(cars: &mut Value, delta_t: f64) {
    util::iterate_mut(cars, |car| do_vel(car, delta_t));
}

pub fn calc_pos(cars: &mut Value, delta_t: f64) {
    util::iterate_mut(cars, |car| do_move(car, delta_t));
}

fn do_coupler(cars: &mut Value) {
    if let Value::Object(cars) = cars {
        let car_keys = cars.keys().cloned().collect::<Vec<_>>();
        for id in &car_keys {
            let next_car = util::get(&cars[id], &["next"]).as_str().map(String::from);
            if let Some(ref next_car) = next_car {
                calc_couplers(cars, id, next_car);
            }
        }
    }
}

fn calc_pull(cars: &mut Value) {
    util::iterate_mut(cars, do_pull)
}

fn do_pull(car: &mut Value) {
    let acc = util::get_num(car, &["acc"]).unwrap();
    let pull = util::get_num(car, &["pull"]).unwrap_or(ZERO);
    let mass = util::get_num(car, &["mass"]).unwrap();
    util::set_num(car, &["acc"], acc + pull / mass);
}

fn calc_friction(cars: &mut Value) {
    util::iterate_mut(cars, do_friction);
}

fn do_friction(car: &mut Value) {
    let mu = util::get_num(car, &["brk"]).unwrap_or(ZERO);
    let mass = util::get_num(car, &["mass"]).unwrap();
    let vel = util::get_num(car, &["vel"]).unwrap();
    let drag = ALPHA * vel / mass + if vel < ZERO { -mu * GRAV } else { mu * GRAV };
    let mut acc = util::get_num(car, &["acc"]).unwrap();
    if vel.abs() > ZERO {
        acc -= drag
    } else {
        let drag = acc.abs().min(2. * drag);
        acc += if acc < ZERO { drag } else { -drag };
    }
    util::set_num(car, &["acc"], acc);
}

fn do_vel(car: &mut Value, delta_t: f64) {
    let v0 = util::get_num(car, &["vel"]).unwrap();
    let acc = util::get_num(car, &["acc"]).unwrap();
    let v_next = v0 + acc * delta_t;
    util::set_num(
        car,
        &["vel"],
        if v0 * v_next < ZERO { ZERO } else { v_next },
    );
}

fn do_move(car: &mut Value, delta_t: f64) {
    let x0 = util::get_num(car, &["pos"]).unwrap();
    let vel = util::get_num(car, &["vel"]).unwrap();
    let acc = util::get_num(car, &["acc"]).unwrap();
    util::set_num(
        car,
        &["pos"],
        x0 + vel * delta_t + 0.5 * acc * delta_t * delta_t,
    );
}

fn calc_couplers(cars: &mut Map<String, Value>, current: &str, next: &str) {
    let pos = util::get_num(&cars[current], &["pos"]).unwrap();
    let pos_next = util::get_num(&cars[next], &["pos"]).unwrap();
    let dx = pos - pos_next;
    let force = -K * (dx.abs() - L0);

    let mass = util::get_num(&cars[current], &["mass"]).unwrap();
    let mass_next = util::get_num(&cars[next], &["mass"]).unwrap();
    let acc = util::get_num(&cars[current], &["acc"]).unwrap();
    let acc_next = util::get_num(&cars[next], &["acc"]).unwrap();
    util::set_num(&mut cars[current], &["acc"], acc + force / mass);
    util::set_num(&mut cars[next], &["acc"], acc_next - force / mass_next);
}
